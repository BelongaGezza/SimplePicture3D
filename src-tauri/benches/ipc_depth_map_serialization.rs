//! ARCH-501: IPC performance spike — measure JSON serialization time for depth map payloads.
//! Payload shape matches DepthMapOutput (width, height, depth) as sent over Tauri invoke.
//! Run: cargo bench --bench ipc_depth_map_serialization (from src-tauri).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DepthMapPayload {
    width: u32,
    height: u32,
    depth: Vec<f32>,
}

fn make_payload(width: u32, height: u32) -> DepthMapPayload {
    let n = (width * height) as usize;
    let depth: Vec<f32> = (0..n).map(|i| (i % 256) as f32 / 255.0).collect();
    DepthMapPayload {
        width,
        height,
        depth,
    }
}

fn bench_serialize(c: &mut Criterion) {
    // 640×480 ≈ 1.2M values → ~5MB JSON
    let p640 = make_payload(640, 480);
    c.bench_function("serialize_depth_map_640x480", |b| {
        b.iter(|| serde_json::to_string(black_box(&p640)).unwrap())
    });

    // 1920×1080 ≈ 2.1M values → ~8MB JSON
    let p1080 = make_payload(1920, 1080);
    c.bench_function("serialize_depth_map_1920x1080", |b| {
        b.iter(|| serde_json::to_string(black_box(&p1080)).unwrap())
    });

    // 3840×2160 ≈ 8.3M values → ~33MB JSON
    let p4k = make_payload(3840, 2160);
    c.bench_function("serialize_depth_map_3840x2160", |b| {
        b.iter(|| serde_json::to_string(black_box(&p4k)).unwrap())
    });
}

criterion_group!(benches, bench_serialize);
criterion_main!(benches);
