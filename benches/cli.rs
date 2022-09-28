use std::fs;
use std::process::Command;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use escargot::CargoBuild;

fn rmd() -> Command {
    CargoBuild::new()
        .bin("rmd")
        .features("auto-interactive")
        .current_release()
        .run()
        .unwrap()
        .command()
}

fn rm() -> Command {
    Command::new("rm")
}

fn rsync() -> Command {
    Command::new("rsync")
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

fn bench_dfs_n_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs n files");
    let mut rmd = rmd();
    let mut rm = rm();
    let mut rsync = rsync();

    let dir = n_files(black_box(1000));
    group.bench_function("rmd: remove folder with n files", |b| {
        b.iter(|| {
            rmd.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });

    let dir = n_files(black_box(1000));
    group.bench_function("rm: remove folder with n files", |b| {
        b.iter(|| {
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });

    let dir = n_files(black_box(1000));
    group.bench_function("rsync: remove folder with n files", |b| {
        b.iter(|| {
            let empty_dir = TempDir::new().unwrap();
            rsync
                .arg("-a")
                .arg("--delete")
                .arg(&empty_dir.path())
                .arg(dir.path())
                .output()
                .expect("to execute rsync");
        })
    });
}

fn bench_dfs_n_nested_folders(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs n nested folders");
    let mut rmd = rmd();
    let mut rm = rm();
    let mut rsync = rsync();

    let dir = n_nested_folder(black_box(200));
    group.bench_function("rmd: remove nested folder depth n", |b| {
        b.iter(|| {
            rmd.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });

    let dir = n_nested_folder(black_box(200));
    group.bench_function("rm: remove nested folder depth n", |b| {
        b.iter(|| {
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });

    let dir = n_nested_folder(black_box(200));
    group.bench_function("rsync: remove nested folder depth n", |b| {
        b.iter(|| {
            let empty_dir = TempDir::new().unwrap();
            rsync
                .arg("-a")
                .arg("--delete")
                .arg(&empty_dir.path())
                .arg(dir.path())
                .output()
                .expect("to execute rsync");
        })
    });
}

fn bench_dfs_m_folders_n_nested_each(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs m nested folder n");
    let mut rmd = rmd();
    let mut rm = rm();
    let mut rsync = rsync();

    let dir = m_nested_folder_n(black_box(20), black_box(200));
    group.bench_function("rmd: remove nested m folders depth n each", |b| {
        b.iter(|| {
            rmd.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });

    let dir = m_nested_folder_n(black_box(20), black_box(200));
    group.bench_function("rm: remove nested m folders depth n each", |b| {
        b.iter(|| {
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });

    let dir = m_nested_folder_n(black_box(20), black_box(200));
    group.bench_function("rsync: -a --delete", |b| {
        b.iter(|| {
            let empty_dir = TempDir::new().unwrap();
            rsync
                .arg("-a")
                .arg("--delete")
                .arg(&empty_dir.path())
                .arg(dir.path())
                .output()
                .expect("to execute rsync");
        })
    });
}

fn bench_rip_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("rip mode");
    let mut rmd = rmd();
    let mut rm = rm();
    let mut rsync = rsync();

    let dir = m_nested_folder_n(black_box(20), black_box(200));
    group.bench_function("rmd: rip mode", |b| {
        b.iter(|| {
            rmd.arg("-x")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });

    let dir = m_nested_folder_n(black_box(20), black_box(200));
    group.bench_function("rm: remove all", |b| {
        b.iter(|| {
            rm.arg("-r")
                .arg("-f")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });

    let dir = m_nested_folder_n(black_box(20), black_box(200));
    group.bench_function("rsync: -a --delete", |b| {
        b.iter(|| {
            let empty_dir = TempDir::new().unwrap();
            rsync
                .arg("-a")
                .arg("--delete")
                .arg(&empty_dir.path())
                .arg(dir.path())
                .output()
                .expect("to execute rsync");
        })
    });
}

criterion_group!(
    benches,
    bench_dfs_n_files,
    bench_dfs_n_nested_folders,
    bench_dfs_m_folders_n_nested_each,
    bench_rip_mode
);
criterion_main!(benches);
