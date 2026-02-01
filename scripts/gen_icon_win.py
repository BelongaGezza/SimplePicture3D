#!/usr/bin/env python3
"""Generate a minimal Windows RC-compatible icon.ico (BMP-based, no PNG compression).
Fixes RC2176 'old DIB' when rc.exe rejects PNG-compressed ICO. Run from repo root."""
import struct
import os

OUT = os.path.join(os.path.dirname(__file__), "..", "src-tauri", "icons", "icon.ico")
os.makedirs(os.path.dirname(OUT), exist_ok=True)

W, H = 32, 32
# ICO header: reserved 0, type 1 (ICO), count 1
ico_header = struct.pack("<HHH", 0, 1, 1)
# Directory entry: width, height, 0, 0, planes 1, bpp 32, size, offset 22
bmp_header_size = 40
pixels_size = W * H * 4
image_size = bmp_header_size + pixels_size
dir_entry = struct.pack("<BBBBHHII", W, H, 0, 0, 1, 32, image_size, 22)
# DIB header (BITMAPINFOHEADER): size 40, width, height (positive = bottom-up), planes 1, bpp 32
dib = struct.pack("<IiiHHIIiiII", 40, W, H, 1, 32, 0, 0, 0, 0, 0, 0)
# Pixels: 32x32 BGRA, bottom-up, 4 bytes per pixel = 4096 bytes
pixels = bytes([0, 0, 180, 255]) * (W * H)
with open(OUT, "wb") as f:
    f.write(ico_header)
    f.write(dir_entry)
    f.write(dib)
    f.write(pixels)
print("Wrote", OUT)
