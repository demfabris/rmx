use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use escargot::CargoBuild;

fn rmx() -> Command {
    CargoBuild::new()
        .bin("rmx")
        .features("auto-interactive")
        .current_release()
        .run()
        .unwrap()
        .command()
}

fn rm() -> Command {
    Command::new("rm")
}

fn rmt() -> Command {
    Command::new("rmt")
}

fn rmd() -> Command {
    Command::new("rmd")
}

fn n_files(n: usize) -> TempDir {
    let dir = TempDir::new().unwrap();

    for i in 0..n {
        dir.child(format!("file{i}")).touch().unwrap();
    }

    dir
}

fn m_nested_folder_n(m: usize, n: usize) -> TempDir {
    let dir = TempDir::new().unwrap();

    for _ in 0..m {
        let nested = n_nested_folder(n);
        dir.child(nested.path()).create_dir_all().unwrap();
    }

    dir
}

fn n_nested_folder(n: usize) -> TempDir {
    let dir = TempDir::new().unwrap();
    let mut curr = dir.path().to_path_buf();

    for i in 0..n {
        let nested = format!("dir{i}");
        let path = curr.join(nested);
        fs::create_dir_all(&path).unwrap();
        curr = path;
    }

    dir
}

fn installed(name: &str) -> bool {
    Command::new(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

fn bench_dfs_n_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs n files");
    group
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);

    let mut rmx = rmx();
    let mut rm = rm();
    let mut rmt = rmt();
    let mut rmd = rmd();

    let dir = n_files(black_box(500));
    group.bench_function("rmx -rf", |b| {
        b.iter(|| {
            rmx.arg("-r")
                .arg("-f")
                .arg(dir.path())
                .output()
                .expect("to execute rmx");
        })
    });

    if installed("rm") {
        let dir = n_files(black_box(500));
        group.bench_function("rm -rf", |b| {
            b.iter(|| {
                rm.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rm");
            })
        });
    }

    if installed("rmt") {
        let dir = n_files(black_box(500));
        group.bench_function("rmt -rf", |b| {
            b.iter(|| {
                rmt.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmt");
            })
        });
    }

    if installed("rmd") {
        let dir = n_files(black_box(500));
        group.bench_function("rmd -rf", |b| {
            b.iter(|| {
                rmd.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmd");
            })
        });
    }
}

fn bench_dfs_n_nested_folders(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs n nested folders");
    group
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);

    let mut rmx = rmx();
    let mut rm = rm();
    let mut rmt = rmt();
    let mut rmd = rmd();

    let dir = n_nested_folder(black_box(100));
    group.bench_function("rmx -rf", |b| {
        b.iter(|| {
            rmx.arg("-r")
                .arg("-f")
                .arg(dir.path())
                .output()
                .expect("to execute rmx");
        })
    });

    if installed("rm") {
        let dir = n_nested_folder(black_box(100));
        group.bench_function("rm -rf", |b| {
            b.iter(|| {
                rm.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rm");
            })
        });
    }

    if installed("rmt") {
        let dir = n_nested_folder(black_box(100));
        group.bench_function("rmt -rf", |b| {
            b.iter(|| {
                rmt.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmt");
            })
        });
    }

    if installed("rmd") {
        let dir = n_nested_folder(black_box(100));
        group.bench_function("rmd -rf", |b| {
            b.iter(|| {
                rmd.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmd");
            })
        });
    }
}

fn bench_dfs_m_folders_n_nested_each(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs m nested folder n");
    group
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);

    let mut rmx = rmx();
    let mut rm = rm();
    let mut rmt = rmt();
    let mut rmd = rmd();

    let dir = m_nested_folder_n(black_box(20), black_box(100));
    group.bench_function("rmx -rf", |b| {
        b.iter(|| {
            rmx.arg("-r")
                .arg("-f")
                .arg(dir.path())
                .output()
                .expect("to execute rmx");
        })
    });

    if installed("rm") {
        let dir = m_nested_folder_n(black_box(20), black_box(100));
        group.bench_function("rm -rf", |b| {
            b.iter(|| {
                rm.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rm");
            })
        });
    }

    if installed("rmt") {
        let dir = m_nested_folder_n(black_box(20), black_box(100));
        group.bench_function("rmt -rf", |b| {
            b.iter(|| {
                rmt.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmt");
            })
        });
    }

    if installed("rmt") {
        let dir = m_nested_folder_n(black_box(20), black_box(100));
        group.bench_function("rmd -rf", |b| {
            b.iter(|| {
                rmd.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmd");
            })
        });
    }
}

fn bench_rip_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("rip mode");
    group
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);

    let mut rmx = rmx();
    let mut rm = rm();
    let mut rmt = rmt();
    let mut rmd = rmd();

    let dir = m_nested_folder_n(black_box(20), black_box(100));
    group.bench_function("rmx --rip", |b| {
        b.iter(|| {
            rmx.arg("-x")
                .arg(dir.path())
                .output()
                .expect("to execute rmx");
        })
    });

    if installed("rm") {
        let dir = m_nested_folder_n(black_box(20), black_box(100));
        group.bench_function("rm -rf", |b| {
            b.iter(|| {
                rm.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rm");
            })
        });
    }

    if installed("rmt") {
        let dir = m_nested_folder_n(black_box(20), black_box(100));
        group.bench_function("rmt -rf", |b| {
            b.iter(|| {
                rmt.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmt");
            })
        });
    }

    if installed("rmt") {
        let dir = m_nested_folder_n(black_box(20), black_box(100));
        group.bench_function("rmd -rf", |b| {
            b.iter(|| {
                rmd.arg("-r")
                    .arg("-f")
                    .arg(dir.path())
                    .output()
                    .expect("to execute rmd");
            })
        });
    }
}

criterion_group!(
    benches,
    bench_dfs_n_files,
    bench_dfs_n_nested_folders,
    bench_dfs_m_folders_n_nested_each,
    bench_rip_mode
);
criterion_main!(benches);
