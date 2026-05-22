pub mod pclp1;
pub mod titleco;
pub mod wcp;

use calamine::Data;

/// Extract a float from a cell; returns 0.0 on empty/string/error.
pub fn cell_f64(cell: &Data) -> f64 {
    match cell {
        Data::Float(f) => *f,
        Data::Int(i) => *i as f64,
        _ => 0.0,
    }
}

/// Extract a string from a cell; returns empty string for non-string.
pub fn cell_str(cell: &Data) -> String {
    match cell {
        Data::String(s) => s.trim().to_string(),
        Data::Float(f) => format!("{f}"),
        Data::Int(i) => i.to_string(),
        _ => String::new(),
    }
}

/// Read a float at (row, col) from a range (0-indexed); 0.0 if out of bounds or empty.
pub fn get_f64(range: &calamine::Range<Data>, row: u32, col: u32) -> f64 {
    range
        .get_value((row, col))
        .map(cell_f64)
        .unwrap_or(0.0)
}

/// Read a string at (row, col); empty string if out of bounds or non-string.
pub fn get_str(range: &calamine::Range<Data>, row: u32, col: u32) -> String {
    range
        .get_value((row, col))
        .map(cell_str)
        .unwrap_or_default()
}
