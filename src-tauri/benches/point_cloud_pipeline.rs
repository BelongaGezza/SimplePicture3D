//! Benchmark: 4K surface-map point cloud generation + PLY export (QA-D-02).
//!
//! Measures Rust-only pipeline time (no Python depth estimation). For end-to-end
//! timing including AI depth, record results in `SPRINTS/Sprint_D/MANUAL_TEST_REPORT.md`.
//!
//! Run from repo root:
//!   cargo bench --manifest-path src-tauri/Cargo.toml --bench point_cloud_pipeline

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simplepicture3d_lib::blank_envelope::BlankEnvelope;
use simplepicture3d_lib::export::{export_ply, ExportMetadata};
use simplepicture3d_lib::volumetric::{generate_volumetric_points, VolumetricParams};

fn synthetic_depth(width: u32, height: u32) -> Vec<f32> {
    let len = (width as usize).saturating_mul(height as usize);
    (0..len)
        .map(|i| (i % 1000) as f32 / 1000.0)
        .collect()
}

fn bench_generate(c: &mut Criterion, label: &str, width: u32, height: u32, step: u32) {
    let depth = synthetic_depth(width, height);
    let envelope = BlankEnvelope::default();
    let params = VolumetricParams {
        step_x: step,
        step_y: step,
        depth_threshold: 0.05,
    };
    c.bench_function(label, |b| {
        b.iter(|| {
            generate_volumetric_points(
                black_box(&depth),
                width,
                height,
                black_box(&params),
                black_box(&envelope),
            )
        })
    });
}

fn bench_generate_and_export_ply(c: &mut Criterion) {
    let width = 3840u32;
    let height = 2160u32;
    let depth = synthetic_depth(width, height);
    let envelope = BlankEnvelope::default();
    let params = VolumetricParams {
        step_x: 1,
        step_y: 1,
        depth_threshold: 0.05,
    };
    let mut group = c.benchmark_group("qa_d02_4k_rust_pipeline");
    group.sample_size(10);
    group.bench_function("generate_and_export_ply", |b| {
        b.iter(|| {
            let result = generate_volumetric_points(
                black_box(&depth),
                width,
                height,
                black_box(&params),
                black_box(&envelope),
            )
            .expect("generate");
            let path = std::env::temp_dir().join(format!(
                "sp3d_bench_{}_{}.ply",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0)
            ));
            let metadata = ExportMetadata {
                source_image: Some("bench_4k.png".to_string()),
                blank_envelope: Some(envelope.clone()),
                ..ExportMetadata::new()
            };
            export_ply(&path, &result.points, &metadata, false).expect("export");
            let _ = std::fs::remove_file(&path);
            result.point_count
        })
    });
    group.finish();
}

fn benches(c: &mut Criterion) {
    bench_generate(c, "surface_map_1080p_step1", 1920, 1080, 1);
    bench_generate(c, "surface_map_4k_step2", 3840, 2160, 2);
    bench_generate(c, "surface_map_4k_step1", 3840, 2160, 1);
    bench_generate_and_export_ply(c);
}

criterion_group!(groups, benches);
criterion_main!(groups);
