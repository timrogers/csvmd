use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use csvmd::{csv_to_markdown, csv_to_markdown_streaming, Config};
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

/// Direct comparison between standard and streaming modes
fn bench_standard_vs_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_vs_streaming");
    
    // Test different dataset sizes for comparison
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
        
        // Benchmark standard mode
        group.bench_with_input(
            BenchmarkId::new("standard", name), 
            &csv_data, 
            |b, data| {
                b.iter(|| {
                    let input = Cursor::new(black_box(data));
                    let config = Config::default();
                    csv_to_markdown(input, config).unwrap()
                });
            }
        );
        
        // Benchmark streaming mode
        group.bench_with_input(
            BenchmarkId::new("streaming", name), 
            &csv_data, 
            |b, data| {
                b.iter(|| {
                    let input = Cursor::new(black_box(data));
                    let mut output = Vec::new();
                    let config = Config::default();
                    csv_to_markdown_streaming(input, &mut output, config).unwrap();
                    black_box(output);
                });
            }
        );
    }
    
    group.finish();
}

/// Comparison with complex data containing special characters
fn bench_complex_data_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_data_comparison");
    
    let rows = 1_000;
    let cols = 5;
    let csv_data = generate_csv_data(rows, cols, true);
    let data_size = csv_data.len();
    
    group.throughput(Throughput::Bytes(data_size as u64));
    
    // Standard mode with complex data
    group.bench_function("standard_complex", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&csv_data));
            let config = Config::default();
            csv_to_markdown(input, config).unwrap()
        });
    });
    
    // Streaming mode with complex data
    group.bench_function("streaming_complex", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&csv_data));
            let mut output = Vec::new();
            let config = Config::default();
            csv_to_markdown_streaming(input, &mut output, config).unwrap();
            black_box(output);
        });
    });
    
    group.finish();
}

/// Comparison with wide tables (many columns)
fn bench_wide_tables_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide_tables_comparison");
    
    let rows = 500;
    let cols = 50;
    let csv_data = generate_csv_data(rows, cols, false);
    let data_size = csv_data.len();
    
    group.throughput(Throughput::Bytes(data_size as u64));
    
    // Standard mode with wide table
    group.bench_function("standard_wide", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&csv_data));
            let config = Config::default();
            csv_to_markdown(input, config).unwrap()
        });
    });
    
    // Streaming mode with wide table
    group.bench_function("streaming_wide", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&csv_data));
            let mut output = Vec::new();
            let config = Config::default();
            csv_to_markdown_streaming(input, &mut output, config).unwrap();
            black_box(output);
        });
    });
    
    group.finish();
}

/// Comparison with uneven column counts
fn bench_uneven_columns_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("uneven_columns_comparison");
    
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
    
    let data_size = uneven_data.len();
    group.throughput(Throughput::Bytes(data_size as u64));
    
    // Standard mode with uneven columns
    group.bench_function("standard_uneven", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&uneven_data));
            let config = Config::default();
            csv_to_markdown(input, config).unwrap()
        });
    });
    
    // Streaming mode with uneven columns
    group.bench_function("streaming_uneven", |b| {
        b.iter(|| {
            let input = Cursor::new(black_box(&uneven_data));
            let mut output = Vec::new();
            let config = Config::default();
            csv_to_markdown_streaming(input, &mut output, config).unwrap();
            black_box(output);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_standard_vs_streaming,
    bench_complex_data_comparison,
    bench_wide_tables_comparison,
    bench_uneven_columns_comparison
);
criterion_main!(benches);