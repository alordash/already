use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const N: u128 = 20;

fn fibonacci_benchmark(c: &mut Criterion) {
    copy_dynamic_lib().unwrap_or_else(|e| panic!("Error copying dynamic library: {e:?}"));

    c.bench_function("regular slow fibonacci", |b| {
        b.iter(|| benchmarks::regular_slow_fibonacci(black_box(N)))
    });

    c.bench_function("hotreload slow fibonacci", |b| {
        b.iter(|| benchmarks::hotreload_slow_fibonacci(black_box(N)))
    });

    c.bench_function("regular fast fibonacci", |b| {
        b.iter(|| benchmarks::regular_fast_fibonacci(black_box(N)))
    });

    c.bench_function("hotreload fast fibonacci", |b| {
        b.iter(|| benchmarks::hotreload_fast_fibonacci(black_box(N)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);

fn copy_dynamic_lib() -> std::io::Result<()> {
    println!("Searching for dynamic library...");
    let current_exe = std::env::current_exe()?;
    let current_dir = current_exe
        .parent()
        .unwrap_or_else(|| panic!("Unable to get parent directory of current exe."));
    let crate_build_dir = current_dir
        .parent()
        .unwrap_or_else(|| panic!("Unable to get hashed directory of crate."))
        .parent()
        .unwrap_or_else(|| panic!("Unable to get crate build directory."));

    let library_platform_name = hotcode::get_platform_library_file_name(env!("CARGO_PKG_NAME"));

    let dynamic_library_path =
        find_dynamic_lib(crate_build_dir, current_dir, &library_platform_name);
    let dynamic_library_file_name = dynamic_library_path.file_name().unwrap_or_else(|| {
        panic!("Unable to get dynamic library file name from path {dynamic_library_path:?}.")
    });
    let target_dynamic_library_path = current_dir.join(dynamic_library_file_name);

    if std::fs::exists(&target_dynamic_library_path)? {
        std::fs::remove_file(&target_dynamic_library_path)?;
    }

    println!("Copying dynamic library from {dynamic_library_path:?} to {current_dir:?}...");
    std::fs::copy(dynamic_library_path, target_dynamic_library_path)?;

    Ok(())
}

fn find_dynamic_lib(dir: &Path, current_dir: &Path, library_platform_name: &str) -> PathBuf {
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if let Some(entry_parent) = entry.path().parent()
            && entry_parent == current_dir
        {
            continue;
        }
        if entry.file_type().is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .eq_ignore_ascii_case(library_platform_name)
        {
            return entry.path().to_owned();
        }
    }

    panic!("Unable to find library '{library_platform_name}' in {dir:?}.");
}
