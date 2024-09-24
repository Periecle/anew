use criterion::{criterion_group, criterion_main, Criterion};
use std::process::{Command, Stdio};
use std::io::Write;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_input_data(size: usize, unique_ratio: f64) -> String {
    let unique_lines_count = (size as f64 * unique_ratio) as usize;
    let unique_lines = (0..unique_lines_count)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>();

    let mut rng = thread_rng();
    let mut lines = Vec::with_capacity(size);
    for _ in 0..size {
        let line = unique_lines.choose(&mut rng).unwrap().clone();
        lines.push(line);
    }

    lines.join("\n")
}

fn execute_anew(input_data: &str) {
    let mut child = Command::new("./anew")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn anew process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input_data.as_bytes()).expect("Failed to write to stdin");
    }

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        eprintln!("anew exited with a non-zero status");
    }
}

fn bench_anew(c: &mut Criterion) {
    let input_sizes = [1_000, 10_000, 100_000];
    let unique_ratios = [0.5, 0.9, 1.0];

    for &size in &input_sizes {
        for &ratio in &unique_ratios {
            let input_data = generate_input_data(size, ratio);
            c.bench_function(
                &format!("anew_size_{}_unique_{:.1}", size, ratio),
                |b| {
                    b.iter(|| execute_anew(&input_data));
                },
            );
        }
    }
}

criterion_group!(benches, bench_anew);
criterion_main!(benches);
