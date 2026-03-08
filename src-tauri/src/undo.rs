// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Undo/redo for depth adjustment and mask state (BACK-1401, BACK-1402, ARCH-403, ARCH-502).
//!
//! Command pattern: each command stores previous and new state; execute applies new,
//! undo restores previous. History limited to last 20 actions (drop oldest when full).
//! Single stack with heterogeneous commands (Depth | Mask) per ARCH-502.

use crate::depth_adjust::DepthAdjustmentParams;
use crate::mask::MaskBitmap;
use std::collections::VecDeque;

/// Maximum number of undo steps (ARCH-403).
pub const MAX_HISTORY_LEN: usize = 20;

/// A single undoable mutation of depth adjustment params.
/// Holds previous state (for undo) and new state (for redo after undo).
#[derive(Debug, Clone)]
pub struct SetDepthParamsCommand {
    pub previous: DepthAdjustmentParams,
    pub new: DepthAdjustmentParams,
}

impl SetDepthParamsCommand {
    /// Apply the "do" state (new params). Caller has already updated state; this is for redo.
    pub fn apply_new(&self, state: &mut DepthAdjustmentParams) {
        *state = self.new.clone();
    }

    /// Apply the "undo" state (previous params).
    pub fn apply_previous(&self, state: &mut DepthAdjustmentParams) {
        *state = self.previous.clone();
    }
}

/// A single undoable mutation of the mask bitmap (ARCH-502).
#[derive(Debug, Clone)]
pub struct SetMaskCommand {
    pub previous: MaskBitmap,
    pub new: MaskBitmap,
}

impl SetMaskCommand {
    /// Apply the "undo" state (previous mask).
    pub fn apply_previous(&self, mask: &mut Option<MaskBitmap>) {
        *mask = Some(self.previous.clone());
    }

    /// Apply the "do" state (new mask). Used for redo.
    pub fn apply_new(&self, mask: &mut Option<MaskBitmap>) {
        *mask = Some(self.new.clone());
    }
}

/// Heterogeneous undoable command (ARCH-502). One stack for both depth and mask.
#[derive(Debug, Clone)]
pub enum UndoableCommand {
    Depth(SetDepthParamsCommand),
    Mask(SetMaskCommand),
}

impl UndoableCommand {
    /// Apply undo (restore previous state) to the relevant state.
    pub fn apply_previous(
        &self,
        params: &mut DepthAdjustmentParams,
        mask: &mut Option<MaskBitmap>,
    ) {
        match self {
            UndoableCommand::Depth(cmd) => cmd.apply_previous(params),
            UndoableCommand::Mask(cmd) => cmd.apply_previous(mask),
        }
    }

    /// Apply redo (restore new state) to the relevant state.
    pub fn apply_new(&self, params: &mut DepthAdjustmentParams, mask: &mut Option<MaskBitmap>) {
        match self {
            UndoableCommand::Depth(cmd) => cmd.apply_new(params),
            UndoableCommand::Mask(cmd) => cmd.apply_new(mask),
        }
    }
}

/// Undo and redo stacks with a fixed capacity (ARCH-403).
/// When undo stack exceeds MAX_HISTORY_LEN, oldest entry is dropped.
pub struct UndoRedoHistory {
    undo_stack: VecDeque<UndoableCommand>,
    redo_stack: Vec<UndoableCommand>,
    max_len: usize,
}

impl UndoRedoHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: Vec::new(),
            max_len: MAX_HISTORY_LEN,
        }
    }

    /// Push a completed command onto the undo stack. Clears redo. Drops oldest if at capacity.
    pub fn push(&mut self, cmd: UndoableCommand) {
        self.redo_stack.clear();
        if self.undo_stack.len() >= self.max_len {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(cmd);
    }

    /// Pop the most recent command for undo. Returns None if nothing to undo.
    pub fn pop_undo(&mut self) -> Option<UndoableCommand> {
        self.undo_stack.pop_back()
    }

    /// Push a command back onto the redo stack (after undo).
    pub fn push_redo(&mut self, cmd: UndoableCommand) {
        self.redo_stack.push(cmd);
    }

    /// Pop from redo stack for redo. Returns None if nothing to redo.
    pub fn pop_redo(&mut self) -> Option<UndoableCommand> {
        self.redo_stack.pop()
    }

    /// Push a command back onto the undo stack (after redo).
    pub fn push_undo(&mut self, cmd: UndoableCommand) {
        if self.undo_stack.len() >= self.max_len {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(cmd);
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Number of entries in the undo stack (for tests and debugging).
    pub fn len_undo(&self) -> usize {
        self.undo_stack.len()
    }

    /// Clear both stacks (clear_history).
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for UndoRedoHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// JR2-1401: Execute command → state changes; undo → state restored.
    #[test]
    fn command_execute_undo_restores_state() {
        let initial = DepthAdjustmentParams::default();
        let mut modified = DepthAdjustmentParams::default();
        modified.brightness = 0.25;
        modified.gamma = 1.5;

        let cmd = SetDepthParamsCommand {
            previous: initial.clone(),
            new: modified.clone(),
        };

        let mut state = initial.clone();
        cmd.apply_new(&mut state);
        assert!(
            (state.brightness - 0.25).abs() < 1e-6,
            "execute should apply new params"
        );
        assert!(
            (state.gamma - 1.5).abs() < 1e-6,
            "execute should apply new params"
        );

        cmd.apply_previous(&mut state);
        assert!(
            (state.brightness - initial.brightness).abs() < 1e-6,
            "undo should restore previous state"
        );
        assert!(
            (state.gamma - initial.gamma).abs() < 1e-6,
            "undo should restore previous state"
        );
    }

    /// JR2-1402: After 21+ actions, history must not grow beyond 20 (oldest dropped).
    #[test]
    fn history_cap_at_max() {
        let mut hist = UndoRedoHistory::new();
        let default = DepthAdjustmentParams::default();

        for i in 0..MAX_HISTORY_LEN + 3 {
            let cmd = SetDepthParamsCommand {
                previous: default.clone(),
                new: DepthAdjustmentParams {
                    brightness: i as f32 * 0.01,
                    ..default.clone()
                },
            };
            hist.push(UndoableCommand::Depth(cmd));
        }
        assert_eq!(
            hist.len_undo(),
            MAX_HISTORY_LEN,
            "history must not exceed 20"
        );
    }

    #[test]
    fn undo_redo_roundtrip() {
        let mut hist = UndoRedoHistory::new();
        let a = DepthAdjustmentParams::default();
        let mut b = DepthAdjustmentParams::default();
        b.brightness = 0.2;
        let mut c = DepthAdjustmentParams::default();
        c.brightness = 0.4;

        hist.push(UndoableCommand::Depth(SetDepthParamsCommand {
            previous: a.clone(),
            new: b.clone(),
        }));
        hist.push(UndoableCommand::Depth(SetDepthParamsCommand {
            previous: b.clone(),
            new: c.clone(),
        }));

        let cmd = hist.pop_undo().unwrap();
        if let UndoableCommand::Depth(d) = &cmd {
            assert!(d.new.brightness - 0.4 < 1e-6);
        } else {
            panic!("expected Depth command");
        }
        hist.push_redo(cmd);
        assert!(hist.can_undo());
        assert!(hist.can_redo());

        let cmd = hist.pop_redo().unwrap();
        if let UndoableCommand::Depth(d) = &cmd {
            assert!(d.new.brightness - 0.4 < 1e-6);
        } else {
            panic!("expected Depth command");
        }
        hist.push_undo(cmd);
        assert!(hist.can_undo());
        assert!(!hist.can_redo());
    }

    #[test]
    fn push_clears_redo() {
        let mut hist = UndoRedoHistory::new();
        let a = DepthAdjustmentParams::default();
        let mut b = DepthAdjustmentParams::default();
        b.brightness = 0.1;
        hist.push(UndoableCommand::Depth(SetDepthParamsCommand {
            previous: a.clone(),
            new: b.clone(),
        }));
        let cmd = hist.pop_undo().unwrap();
        hist.push_redo(cmd);
        assert!(hist.can_redo());
        let mut c = DepthAdjustmentParams::default();
        c.brightness = 0.2;
        hist.push(UndoableCommand::Depth(SetDepthParamsCommand {
            previous: b.clone(),
            new: c.clone(),
        }));
        assert!(!hist.can_redo());
    }
}
