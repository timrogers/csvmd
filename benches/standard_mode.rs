use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use csvmd::{csv_to_markdown, Config};
use std::io::Cursor;

/// Generate test CSV data with specified number of rows and columns
fn generate_csv_data(rows: usize, cols: usize, complex: bool) -> String {
    let mut data = String::new();
    
    // Header row
    for i in 0..cols {
        if i > 0 { data.push(','); }
        data.push_str(&format!("Column{}", i + 1));
    }
    data.push('\n');
    
    // Data rows
    for row in 0..rows {
        for col in 0..cols {
            if col > 0 { data.push(','); }
            
            if complex {
                // Add complexity with special characters, pipes, newlines
                match (row + col) % 5 {
                    0 => data.push_str(&format!("Value{}", row * cols + col)),
                    1 => data.push_str(&format!("\"With | pipe {}\",", row)),
                    2 => data.push_str(&format!("\"Line1\nLine2 {}\",", row)),
                    3 => data.push_str(&format!("Unicode★{}", row)),
                    _ => data.push_str(&format!("Simple{}", row * cols + col)),
                }
            } else {
                data.push_str(&format!("Value{}", row * cols + col));
            }
        }
        data.push('\n');
    }
    
    data
}

/// Benchmark standard mode with different dataset sizes
fn bench_standard_mode_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_mode_dataset_sizes");
    
    // Test different dataset sizes
    let sizes = [
        (100, "Small (100 rows)"),
        (1_000, "Medium (1K rows)"),
        (10_000, "Large (10K rows)"),
    ];
    
    for (rows, name) in sizes.iter() {
        let csv_data = generate_csv_data(*rows, 3, false);
        let data_size = csv_data.len();
        
        group.throughput(Throughput::Bytes(data_size as u64));
        group.throughput(Throughput::Elements(*rows as u64));
        
        group.bench_with_input(BenchmarkId::new("rows", name), rows, |b, _| {
            b.iter(|| {
                let input = Cursor::new(black_box(&csv_data));
                let config = Config::default();
                csv_to_markdown(input, config).unwrap()
            });
        });
    }
    
    group.finish();
}

/// Benchmark standard mode with different data complexities
fn bench_standard_mode_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_mode_complexity");
    
    let rows = 1_000;
    let cols = 3;
    
    // Simple data
    let simple_data = generate_csv_data(rows, cols, false);
    group.bench_function("simple_data", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&simple_data));
            let config = Config::default();
            csv_to_markdown(input, config).unwrap()
        });
    });
    
    // Complex data with special characters
    let complex_data = generate_csv_data(rows, cols, true);
    group.bench_function("complex_data", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&complex_data));
            let config = Config::default();
            csv_to_markdown(input, config).unwrap()
        });
    });
    
    group.finish();
}

/// Benchmark standard mode with different column counts
fn bench_standard_mode_wide_tables(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_mode_wide_tables");
    
    let rows = 500;
    let column_counts = [3, 10, 25, 50];
    
    for cols in column_counts.iter() {
        let csv_data = generate_csv_data(rows, *cols, false);
        
        group.bench_with_input(BenchmarkId::new("columns", cols), cols, |b, _| {
            b.iter(|| {
                let input = Cursor::new(black_box(&csv_data));
                let config = Config::default();
                csv_to_markdown(input, config).unwrap()
            });
        });
    }
    
    group.finish();
}

/// Benchmark standard mode with uneven column counts
fn bench_standard_mode_uneven_columns(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_mode_uneven_columns");
    
    // Generate CSV with uneven column counts
    let mut uneven_data = String::from("A,B,C\n");
    for i in 0..1000 {
        match i % 4 {
            0 => uneven_data.push_str(&format!("Value{}\n", i)),
            1 => uneven_data.push_str(&format!("Value{},Value{}\n", i, i + 1)),
            2 => uneven_data.push_str(&format!("Value{},Value{},Value{}\n", i, i + 1, i + 2)),
            _ => uneven_data.push_str(&format!("Value{},Value{},Value{},Value{}\n", i, i + 1, i + 2, i + 3)),
        }
    }
    
    group.bench_function("uneven_columns", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&uneven_data));
            let config = Config::default();
            csv_to_markdown(input, config).unwrap()
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_standard_mode_sizes,
    bench_standard_mode_complexity,
    bench_standard_mode_wide_tables,
    bench_standard_mode_uneven_columns
);
criterion_main!(benches);