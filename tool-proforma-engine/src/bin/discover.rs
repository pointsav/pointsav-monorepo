/// Structural discovery tool — dumps sheet names, dimensions, and header rows/columns.
/// Modes:
///   discover <file>                          — list all sheets + dimensions
///   discover <file> <sheet>                  — row labels (col A) + cols B-G sample
///   discover <file> <sheet> <row> <row>      — same, bounded
///   discover <file> <sheet> full <row> <row> — show ALL columns for those rows
///   discover <file> <sheet> cell <R> <C>     — show one cell (1-indexed row, col letter)
use calamine::{open_workbook, Data, DataType, Reader, Xlsx};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage:\n  discover <file>\n  discover <file> <sheet> [row_start] [row_end]\n  discover <file> <sheet> full <row_start> <row_end>\n  discover <file> <sheet> cell <row> <col_letter>"
        );
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);
    let mut wb: Xlsx<_> = open_workbook(&path).unwrap_or_else(|e| {
        eprintln!("error opening {:?}: {e}", path);
        std::process::exit(1);
    });

    let sheet_names: Vec<String> = wb.sheet_names().to_vec();

    if args.len() == 2 {
        println!("=== {} ===", path.file_name().unwrap().to_string_lossy());
        for name in &sheet_names {
            match wb.worksheet_range(name) {
                Ok(range) => {
                    let (rows, cols) = range.get_size();
                    println!("  sheet {:?}  {} rows × {} cols", name, rows, cols);
                }
                Err(e) => println!("  sheet {:?}  error: {e}", name),
            }
        }
        return;
    }

    let sheet = &args[2];
    let range = wb.worksheet_range(sheet).unwrap_or_else(|e| {
        eprintln!("error reading sheet {:?}: {e}", sheet);
        std::process::exit(1);
    });
    let (nrows, ncols) = range.get_size();

    // Mode: cell <row> <col_letter>
    if args.get(3).map(|s| s.as_str()) == Some("cell") {
        let row: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);
        let col_str = args.get(5).unwrap_or(&"A".to_string()).to_uppercase();
        let col = col_from_str(&col_str);
        let cell = range.get_value((row as u32 - 1, col as u32));
        println!(
            "[{}{}] = {}",
            col_str,
            row,
            cell.map(cell_str).unwrap_or_else(|| "(empty)".into())
        );
        return;
    }

    // Mode: full <row_start> <row_end>
    let full_mode = args.get(3).map(|s| s.as_str()) == Some("full");
    let (row_start, row_end): (usize, usize) = if full_mode {
        let rs: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);
        let re: usize = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(rs + 40);
        (rs, re)
    } else {
        let rs: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
        let re: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(rs + 40);
        (rs, re)
    };

    println!(
        "Sheet {:?}: {} rows × {} cols (cols A–{})",
        sheet,
        nrows,
        ncols,
        col_letter(ncols.saturating_sub(1))
    );
    println!();

    // Header row
    print!("ROW 1 (headers): ");
    if let Some(header_row) = range.rows().next() {
        for (ci, cell) in header_row.iter().enumerate() {
            if !cell.is_empty() {
                print!("[{}]{} ", col_letter(ci), cell_str(cell));
            }
        }
    }
    println!();
    println!();

    let start = row_start.saturating_sub(1);
    let end = row_end.min(nrows);

    if full_mode {
        println!("FULL ROWS {}–{} (all non-empty cells):", row_start, row_end);
        for (ri, row) in range.rows().enumerate().skip(start).take(end - start) {
            let mut line = format!("  row {:>3}: ", ri + 1);
            let mut any = false;
            for (ci, cell) in row.iter().enumerate() {
                if !cell.is_empty() {
                    line.push_str(&format!("[{}]{} ", col_letter(ci), cell_str(cell)));
                    any = true;
                }
            }
            if any {
                println!("{}", line);
            }
        }
    } else {
        println!("ROWS {}–{} (col A + cols B–G sample):", row_start, row_end);
        for (ri, row) in range.rows().enumerate().skip(start).take(end - start) {
            let label = row.first().map(cell_str).unwrap_or_default();
            let mut values = String::new();
            for ci in 1..ncols.min(7) {
                if let Some(cell) = row.get(ci) {
                    if !cell.is_empty() {
                        values.push_str(&format!("  [{}]{}", col_letter(ci), cell_str(cell)));
                    }
                }
            }
            if !label.is_empty() || !values.is_empty() {
                println!("  row {:>3}: {:<42}{}", ri + 1, label, values);
            }
        }
    }
}

fn cell_str(cell: &Data) -> String {
    match cell {
        Data::String(s) => format!("\"{}\"", s.replace('\n', "↵")),
        Data::Float(f) => format!("{f:.4}"),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => format!("{dt:?}"),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        Data::Error(e) => format!("ERR:{e:?}"),
        Data::Empty => String::new(),
    }
}

fn col_letter(idx: usize) -> String {
    let mut n = idx;
    let mut s = String::new();
    loop {
        s.insert(0, (b'A' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    s
}

fn col_from_str(s: &str) -> usize {
    s.chars()
        .fold(0usize, |acc, c| acc * 26 + (c as u8 - b'A') as usize + 1)
        - 1
}
