// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

//! Mask bitmap for regional depth adjustments (BACK-1201, ARCH-502).
//! Row-major packed bits; dimensions must match the current depth map.

/// 2D boolean mask matching depth map dimensions. Stored as packed bits (1 bit per pixel).
#[derive(Debug, Clone)]
pub struct MaskBitmap {
    width: u32,
    height: u32,
    /// Row-major packed bits: bit index = y * width + x; byte index = (y * width + x) / 8.
    data: Vec<u8>,
}

impl MaskBitmap {
    /// Size in bytes for given dimensions.
    fn byte_len(width: u32, height: u32) -> usize {
        let n = (width as usize).saturating_mul(height as usize);
        n.div_ceil(8)
    }

    /// Create a new mask of the given dimensions, all bits false.
    pub fn all_false(width: u32, height: u32) -> Self {
        let len = Self::byte_len(width, height);
        Self {
            width,
            height,
            data: vec![0u8; len],
        }
    }

    /// Dimensions (width, height).
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get value at (x, y). Returns false if out of bounds.
    pub fn get(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let i = (y as usize) * (self.width as usize) + (x as usize);
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        (self.data[byte_idx] >> bit_idx) & 1 != 0
    }

    /// Set value at (x, y). No-op if out of bounds.
    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = (y as usize) * (self.width as usize) + (x as usize);
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        if value {
            self.data[byte_idx] |= 1u8 << bit_idx;
        } else {
            self.data[byte_idx] &= !(1u8 << bit_idx);
        }
    }

    /// Set all pixels in the rectangle [x, x+width) × [y, y+height) to `value`.
    /// Clips to mask bounds.
    pub fn set_region(&mut self, x: u32, y: u32, w: u32, h: u32, value: bool) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for py in y..y_end {
            for px in x..x_end {
                self.set(px, py, value);
            }
        }
    }

    /// Flatten to row-major Vec<bool> for IPC (JSON). Used by get_mask response.
    pub fn to_bool_vec(&self) -> Vec<bool> {
        let n = (self.width as usize).saturating_mul(self.height as usize);
        (0..n).map(|i| self.get_at_index(i)).collect()
    }

    fn get_at_index(&self, i: usize) -> bool {
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        if byte_idx >= self.data.len() {
            return false;
        }
        (self.data[byte_idx] >> bit_idx) & 1 != 0
    }

    /// Check that (width, height) match the mask dimensions.
    pub fn dimensions_match(&self, width: u32, height: u32) -> bool {
        self.width == width && self.height == height
    }

    /// Soft mask for feathering (BACK-1203). Returns row-major Vec<f32> in [0, 1].
    /// When feather_radius_px <= 0, returns 0.0/1.0 per pixel (no blur).
    /// Otherwise box-blurs the boolean mask so edges get values in (0, 1).
    pub fn to_soft_mask(&self, feather_radius_px: f32) -> Vec<f32> {
        let n = (self.width as usize).saturating_mul(self.height as usize);
        let mut out = vec![0.0f32; n];
        let r = feather_radius_px.max(0.0).floor() as u32;
        if r == 0 {
            for (i, v) in out.iter_mut().enumerate().take(n) {
                let x = (i % self.width as usize) as u32;
                let y = (i / self.width as usize) as u32;
                *v = if self.get(x, y) { 1.0 } else { 0.0 };
            }
            return out;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0u32;
                let y0 = y.saturating_sub(r);
                let y1 = (y + r + 1).min(self.height);
                let x0 = x.saturating_sub(r);
                let x1 = (x + r + 1).min(self.width);
                let count = (x1 - x0) * (y1 - y0);
                for py in y0..y1 {
                    for px in x0..x1 {
                        if self.get(px, py) {
                            sum += 1;
                        }
                    }
                }
                let i = (y as usize) * (self.width as usize) + (x as usize);
                out[i] = if count > 0 {
                    (sum as f32 / count as f32).clamp(0.0, 1.0)
                } else {
                    0.0
                };
            }
        }
        out
    }

    /// Build mask from row-major bool slice (JR1-1203, lasso/fill). Fails if data len != width*height.
    pub fn from_bool_vec(width: u32, height: u32, data: &[bool]) -> Result<Self, String> {
        let expected = (width as usize).saturating_mul(height as usize);
        if data.len() != expected {
            return Err(format!(
                "Mask data length {} does not match dimensions {}x{}",
                data.len(),
                width,
                height
            ));
        }
        let mut m = Self::all_false(width, height);
        for (i, &v) in data.iter().enumerate() {
            if i >= expected {
                break;
            }
            let x = (i % (width as usize)) as u32;
            let y = (i / (width as usize)) as u32;
            m.set(x, y, v);
        }
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_false_and_set_get() {
        let mut m = MaskBitmap::all_false(10, 10);
        assert!(!m.get(0, 0));
        m.set(0, 0, true);
        assert!(m.get(0, 0));
        m.set(5, 5, true);
        assert!(m.get(5, 5));
        assert!(!m.get(5, 6));
    }

    #[test]
    fn set_region() {
        let mut m = MaskBitmap::all_false(8, 8);
        m.set_region(2, 2, 3, 3, true);
        for y in 2..5 {
            for x in 2..5 {
                assert!(m.get(x, y), "({}, {})", x, y);
            }
        }
        assert!(!m.get(1, 2));
        assert!(!m.get(5, 4));
    }

    #[test]
    fn dimensions_match() {
        let m = MaskBitmap::all_false(100, 50);
        assert!(m.dimensions_match(100, 50));
        assert!(!m.dimensions_match(100, 51));
        assert!(!m.dimensions_match(99, 50));
    }

    #[test]
    fn from_bool_vec_roundtrip() {
        let w = 4u32;
        let h = 3u32;
        let data: Vec<bool> = (0..(w * h) as usize).map(|i| i % 2 == 0).collect();
        let m = MaskBitmap::from_bool_vec(w, h, &data).unwrap();
        assert_eq!(m.dimensions(), (w, h));
        let back = m.to_bool_vec();
        assert_eq!(back, data);
    }

    #[test]
    fn from_bool_vec_wrong_length() {
        assert!(MaskBitmap::from_bool_vec(2, 2, &[true, false]).is_err());
        assert!(MaskBitmap::from_bool_vec(2, 2, &[true; 5]).is_err());
    }

    /// BACK-1203: to_soft_mask with radius 0 returns 0.0 or 1.0; with radius > 0 edges get values in (0, 1).
    #[test]
    fn to_soft_mask_hard_edge() {
        let mut m = MaskBitmap::all_false(4, 4);
        m.set_region(1, 1, 2, 2, true);
        let soft = m.to_soft_mask(0.0);
        assert_eq!(soft.len(), 16);
        assert_eq!(soft[0], 0.0);
        assert_eq!(soft[5], 1.0); // (1,1) row-major
        assert_eq!(soft[10], 1.0);
    }

    #[test]
    fn to_soft_mask_feather_blur() {
        let mut m = MaskBitmap::all_false(5, 5);
        m.set(2, 2, true);
        let soft = m.to_soft_mask(1.0);
        assert_eq!(soft.len(), 25);
        // With radius 1, 3x3 box has 9 cells; only center is set → center = 1/9
        let center = (2_usize) * 5 + 2;
        assert!(
            soft[center] > 0.0,
            "center should get some weight from masked pixel"
        );
        // Feather produces values in (0, 1) at/near edges
        let has_mid = soft.iter().any(|&v| v > 0.0 && v < 1.0);
        assert!(has_mid, "feather should produce values in (0, 1)");
    }
}
