use std::fmt;

pub trait Table<T: fmt::Display, U: fmt::Display> {
    fn table(&self) -> (T, U);
}

pub fn fmt_table<T: fmt::Display, U: fmt::Display>(rows: Vec<impl Table<T, U>>) -> String {
    let (keys, values): (Vec<_>, Vec<_>) = rows
        .iter()
        .map(|row| (format!("{}", row.table().0), format!("{}", row.table().1)))
        .unzip();

    let key_width = keys.iter().map(|k| k.len()).max().unwrap_or(0);
    let val_width = values.iter().map(|v| v.len()).max().unwrap_or(0);

    let formatted_rows: Vec<String> = keys
        .iter()
        .zip(values.iter())
        .map(|(key, value)| {
            format!(
                "│ {:<width1$} │ {:<width2$} │",
                key,
                value,
                width1 = key_width,
                width2 = val_width
            )
        })
        .collect();

    let top_border = format!("┌─{}─┬─{}─┐", "─".repeat(key_width), "─".repeat(val_width),);
    let bottom_border = format!("└─{}─┴─{}─┘", "─".repeat(key_width), "─".repeat(val_width),);

    format!(
        "{}\n{}\n{}",
        top_border,
        formatted_rows.join("\n"),
        bottom_border,
    )
}
