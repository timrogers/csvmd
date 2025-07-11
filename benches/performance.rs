use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use csvmd::{csv_to_markdown, csv_to_markdown_streaming, Config};
use std::io::Cursor;

// Generate test CSV data of various sizes
fn generate_csv_data(rows: usize, cols: usize) -> String {
    let mut data = String::new();
    
    // Header
    for i in 0..cols {
        if i > 0 {
            data.push(',');
        }
        data.push_str(&format!("Column{}", i + 1));
    }
    data.push('\n');
    
    // Data rows
    for row in 0..rows {
        for col in 0..cols {
            if col > 0 {
                data.push(',');
            }
            data.push_str(&format!("Row{}Col{}", row + 1, col + 1));
        }
        data.push('\n');
    }
    
    data
}

// Generate CSV with complex data (pipes, newlines, quotes)
fn generate_complex_csv_data(rows: usize) -> String {
    let mut data = String::new();
    data.push_str("Name,Description,Value\n");
    
    for i in 0..rows {
        data.push_str(&format!(
            "\"User {}\",\"Description with | pipe and\nnewline\",\"Value: {}\"\n",
            i + 1,
            i * 100
        ));
    }
    
    data
}

fn bench_standard_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_mode");
    
    for &size in &[100, 1000, 10000] {
        let csv_data = generate_csv_data(size, 5);
        let data_size = csv_data.len();
        
        group.throughput(Throughput::Bytes(data_size as u64));
        group.bench_with_input(
            BenchmarkId::new("rows", size),
            &csv_data,
            |b, data| {
                b.iter(|| {
                    let input = Cursor::new(data.as_bytes());
                    let config = Config::default();
                    csv_to_markdown(black_box(input), black_box(config)).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_streaming_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_mode");
    
    for &size in &[100, 1000, 10000] {
        let csv_data = generate_csv_data(size, 5);
        let data_size = csv_data.len();
        
        group.throughput(Throughput::Bytes(data_size as u64));
        group.bench_with_input(
            BenchmarkId::new("rows", size),
            &csv_data,
            |b, data| {
                b.iter(|| {
                    let input = Cursor::new(data.as_bytes());
                    let mut output = Vec::new();
                    let config = Config::default();
                    csv_to_markdown_streaming(black_box(input), black_box(&mut output), black_box(config)).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_complex_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_data");
    
    for &size in &[100, 1000, 5000] {
        let csv_data = generate_complex_csv_data(size);
        let data_size = csv_data.len();
        
        group.throughput(Throughput::Bytes(data_size as u64));
        group.bench_with_input(
            BenchmarkId::new("rows", size),
            &csv_data,
            |b, data| {
                b.iter(|| {
                    let input = Cursor::new(data.as_bytes());
                    let config = Config::default();
                    csv_to_markdown(black_box(input), black_box(config)).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_column_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_scaling");
    
    for &cols in &[5, 10, 20, 50] {
        let csv_data = generate_csv_data(1000, cols);
        let data_size = csv_data.len();
        
        group.throughput(Throughput::Bytes(data_size as u64));
        group.bench_with_input(
            BenchmarkId::new("columns", cols),
            &csv_data,
            |b, data| {
                b.iter(|| {
                    let input = Cursor::new(data.as_bytes());
                    let config = Config::default();
                    csv_to_markdown(black_box(input), black_box(config)).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_standard_mode,
    bench_streaming_mode,
    bench_complex_data,
    bench_column_scaling
);
criterion_main!(benches);
