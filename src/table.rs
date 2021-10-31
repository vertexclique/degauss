#![allow(dead_code)]
use crate::compat::DegaussCompatMode;
use comfy_table::Table;
use std::collections::HashMap;

/// Render the HashMap with its values
pub(crate) fn render(payload: &HashMap<DegaussCompatMode, bool>) {
    let mut table = Table::new();
    table.set_header(vec!["CheckType", "Status"]);
    for (key, value) in payload.iter() {
        table.add_row(vec![key.to_string(), value.to_string()]);
    }
    println!("{}", table);
}
