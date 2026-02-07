use crate::{config::FormatConfig, Result};

pub fn fmt_table(table: &mut toml_edit::Table, config: FormatConfig) -> Result<()> {
    let keys = table
        .clone()
        .iter()
        .map(|(key, _)| key.to_owned())
        .collect::<Vec<String>>();

    for key in &keys {
        if table[key].is_table() {
            fmt_table(table[key.as_str()].as_table_mut().unwrap(), config)?;
        } else if table[key].is_array_of_tables() {
            fmt_array_of_tables(table[key.as_str()].as_array_of_tables_mut().unwrap(), config)?;
        } else if table[key].is_value() {
            fmt_value(table[key.as_str()].as_value_mut().unwrap())?;
        }
    }

    if config.sort_keys {
        table.sort_values();
    }

    Ok(())
}

pub fn fmt_array_of_tables(
    array_of_tables: &mut toml_edit::ArrayOfTables,
    config: FormatConfig,
) -> Result<()> {
    for idx in 0..array_of_tables.len() {
        let table = array_of_tables.get_mut(idx).unwrap();
        fmt_table(table, config)?;
    }

    Ok(())
}

pub fn fmt_value(value: &mut toml_edit::Value) -> Result<()> {
    if value.is_array() {
        value.as_array_mut().unwrap().fmt();
    } else if value.is_inline_table() {
        value.as_inline_table_mut().unwrap().fmt();
    }

    Ok(())
}
