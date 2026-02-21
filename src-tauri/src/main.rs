// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    simplepicture3d_lib::run()
}
