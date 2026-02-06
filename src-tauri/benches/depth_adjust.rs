//! Benchmark: apply depth adjustments to 1920×1080 depth array (JR2-403).
//! Target: <100 ms for real-time preview feasibility.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simplepicture3d_lib::depth_adjust::{apply_adjustments, DepthAdjustmentParams};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const LEN: usize = WIDTH * HEIGHT;

fn bench_apply_adjustments_1080p(c: &mut Criterion) {
    let depth: Vec<f32> = (0..LEN).map(|i| (i % 256) as f32 / 255.0).collect();
    let params = DepthAdjustmentParams {
        brightness: 0.05,
        contrast: 1.2,
        gamma: 1.1,
        invert: false,
        ..Default::default()
    };
    c.bench_function("apply_adjustments_1920x1080", |b| {
        b.iter(|| apply_adjustments(black_box(&depth), black_box(&params)))
    });
}

criterion_group!(benches, bench_apply_adjustments_1080p);
criterion_main!(benches);
