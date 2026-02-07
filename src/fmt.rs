use crate::{config::FormatConfig, Result};

pub fn fmt_table(
    table: &mut toml_edit::Table,
    config: FormatConfig,
    table_name: Option<&str>,
) -> Result<()> {
    let keys = table
        .clone()
        .iter()
        .map(|(key, _)| key.to_owned())
        .collect::<Vec<String>>();

    for key in &keys {
        if table[key].is_table() {
            fmt_table(table[key.as_str()].as_table_mut().unwrap(), config, Some(key))?;
        } else if table[key].is_array_of_tables() {
            fmt_array_of_tables(
                table[key.as_str()].as_array_of_tables_mut().unwrap(),
                config,
                Some(key),
            )?;
        } else if table[key].is_value() {
            if table_name == Some("workspace") && key == "members" {
                if let Some(array) = table[key]
                    .as_value_mut()
                    .and_then(|value| value.as_array_mut())
                {
                    for value in array.iter_mut() {
                        let decor = value.decor_mut();
                        decor.set_prefix("\n    ");
                        decor.set_suffix("");
                    }
                    array.set_trailing_comma(true);
                    array.set_trailing("\n");
                    continue;
                }
            }
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
    table_name: Option<&str>,
) -> Result<()> {
    for idx in 0..array_of_tables.len() {
        let table = array_of_tables.get_mut(idx).unwrap();
        fmt_table(table, config, table_name)?;
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
