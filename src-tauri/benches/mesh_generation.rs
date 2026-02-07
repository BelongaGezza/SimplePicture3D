//! Benchmark: mesh (point cloud) generation from depth map (JR2-503).
//! Target: <15 s for 4K (3840×2160) per PRD §7.1.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simplepicture3d_lib::mesh_generator::{depth_to_point_cloud, MeshParams};

fn make_depth(width: usize, height: usize) -> Vec<f32> {
    let len = width * height;
    (0..len).map(|i| (i % 256) as f32 / 255.0).collect()
}

fn bench_mesh_1k(c: &mut Criterion) {
    let (w, h) = (1024, 1024);
    let depth = make_depth(w, h);
    let params = MeshParams::default();
    c.bench_function("mesh_generation_1024x1024", |b| {
        b.iter(|| {
            depth_to_point_cloud(
                black_box(&depth),
                black_box(w as u32),
                black_box(h as u32),
                black_box(&params),
            )
            .unwrap()
        })
    });
}

fn bench_mesh_4k(c: &mut Criterion) {
    let (w, h) = (3840, 2160);
    let depth = make_depth(w, h);
    let params = MeshParams::default();
    c.bench_function("mesh_generation_3840x2160_4k", |b| {
        b.iter(|| {
            depth_to_point_cloud(
                black_box(&depth),
                black_box(w as u32),
                black_box(h as u32),
                black_box(&params),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, bench_mesh_1k, bench_mesh_4k);
criterion_main!(benches);
