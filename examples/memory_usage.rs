use csvmd::{csv_to_markdown, csv_to_markdown_streaming, Config};
use std::io::Cursor;

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

fn main() {
    // Test memory usage patterns
    let sizes = vec![
        (1000, 5),    // 1K rows, 5 cols
        (10000, 5),   // 10K rows, 5 cols  
        (50000, 5),   // 50K rows, 5 cols
        (100000, 5),  // 100K rows, 5 cols
    ];
    
    for (rows, cols) in sizes {
        let csv_data = generate_csv_data(rows, cols);
        let input_size = csv_data.len();
        
        println!("Testing {} rows × {} columns ({:.1} KB input)", 
                 rows, cols, input_size as f64 / 1024.0);
        
        // Standard mode
        let input = Cursor::new(csv_data.as_bytes());
        let config = Config::default();
        let result = csv_to_markdown(input, config).unwrap();
        let output_size = result.len();
        
        // Streaming mode
        let input = Cursor::new(csv_data.as_bytes());
        let mut output_stream = Vec::new();
        let config = Config::default();
        csv_to_markdown_streaming(input, &mut output_stream, config).unwrap();
        
        println!("  Input size:  {:>10} bytes ({:.1} KB)", 
                 input_size, input_size as f64 / 1024.0);
        println!("  Output size: {:>10} bytes ({:.1} KB)", 
                 output_size, output_size as f64 / 1024.0);
        println!("  Memory ratio: {:.2}x", output_size as f64 / input_size as f64);
        println!();
    }
}
