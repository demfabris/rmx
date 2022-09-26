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

fn tempdir_with_n_files(n: usize) -> TempDir {
    let dir = TempDir::new().unwrap();

    for i in 0..n {
        dir.child(format!("file{i}")).touch().unwrap();
    }

    dir
}

fn tempdir_with_m_childs_nested_n_each(m: usize, n: usize) -> TempDir {
    let dir = TempDir::new().unwrap();

    for _ in 0..m {
        let nested = tempdir_with_n_nested_children(n);
        dir.child(nested.path()).create_dir_all().unwrap();
    }

    dir
}

fn tempdir_with_n_nested_children(n: usize) -> TempDir {
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

fn bench_single_threaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs");
    let mut rmd = rmd();
    let mut rm = rm();

    group.bench_function("rmd: remove folder with n files", |b| {
        b.iter(|| {
            let dir = tempdir_with_n_files(black_box(1000));
            rmd.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });
    group.bench_function("rm: remove folder with n files", |b| {
        b.iter(|| {
            let dir = tempdir_with_n_files(black_box(1000));
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });

    group.bench_function("rmd: remove nested folder depth n", |b| {
        b.iter(|| {
            let dir = tempdir_with_n_nested_children(black_box(200));
            rmd.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });
    group.bench_function("rm: remove nested folder depth n", |b| {
        b.iter(|| {
            let dir = tempdir_with_n_nested_children(black_box(200));
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });

    group.bench_function("rmd: remove nested m folders depth n each", |b| {
        b.iter(|| {
            let dir = tempdir_with_m_childs_nested_n_each(black_box(20), black_box(200));
            rmd.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });
    group.bench_function("rm: remove nested m folders depth n each", |b| {
        b.iter(|| {
            let dir = tempdir_with_m_childs_nested_n_each(black_box(20), black_box(200));
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });
}

fn bench_async_rt(c: &mut Criterion) {
    let mut group = c.benchmark_group("rip");
    let mut rmd = rmd();
    let mut rm = rm();

    group.bench_function("rmd: remove all", |b| {
        b.iter(|| {
            let dir = tempdir_with_m_childs_nested_n_each(black_box(20), black_box(200));
            rmd.arg("-r")
                .arg("-x")
                .arg(dir.path())
                .output()
                .expect("to execute rmd");
        })
    });
    group.bench_function("rm: remove all", |b| {
        b.iter(|| {
            let dir = tempdir_with_m_childs_nested_n_each(black_box(20), black_box(200));
            rm.arg("-r")
                .arg(dir.path())
                .output()
                .expect("to execute rm");
        })
    });
}

criterion_group!(benches, bench_single_threaded, bench_async_rt);
criterion_main!(benches);
