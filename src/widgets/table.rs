use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<T: fmt::Display, U: fmt::Display> {
    pub rows: Vec<(T, U)>,
}

impl<T: fmt::Display, U: fmt::Display> Table<T, U>
where
    T: fmt::Display,
    U: fmt::Display,
{
    pub fn new(rows: Vec<(T, U)>) -> Self {
        Self { rows }
    }
}

impl<T, U> Iterator for Table<T, U>
where
    T: fmt::Display,
    U: fmt::Display,
{
    type Item = (T, U);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rows.is_empty() {
            None
        } else {
            Some(self.rows.remove(0))
        }
    }
}

impl<T, U, I> FromIterator<I> for Table<T, U>
where
    T: fmt::Display + Clone,
    U: fmt::Display + Clone,
    I: Into<Table<T, U>>,
{
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: IntoIterator<Item = I>,
    {
        let mut merged_rows = Vec::new();
        for item in iter {
            let table = item.into();
            merged_rows.extend(table.rows);
        }
        Table::new(merged_rows)
    }
}

impl<T: fmt::Display, U: fmt::Display> From<(T, U)> for Table<T, U> {
    fn from(value: (T, U)) -> Self {
        Self { rows: vec![value] }
    }
}

impl<T, U> fmt::Display for Table<T, U>
where
    T: fmt::Display,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (keys, values): (Vec<_>, Vec<_>) = self
            .rows
            .iter()
            .map(|row| (row.0.to_string(), row.1.to_string()))
            .unzip();
        let key_width = keys.iter().map(|k| k.len()).max().unwrap_or(0);
        let val_width = values.iter().map(|v| v.len()).max().unwrap_or(0);

        let formatted_rows: Vec<String> = keys
            .iter()
            .zip(values.iter())
            .map(|(key, value)| {
                format!(
                    "│ {:<width$} │ {:<val_width$} │",
                    key,
                    value,
                    width = key_width,
                    val_width = val_width
                )
            })
            .collect();

        let top_border = format!("┌─{}─┬─{}─┐", "─".repeat(key_width), "─".repeat(val_width),);
        let bottom_border = format!("└─{}─┴─{}─┘", "─".repeat(key_width), "─".repeat(val_width),);

        write!(
            f,
            "{}\n{}\n{}",
            top_border,
            formatted_rows.join("\n"),
            bottom_border,
        )
    }
}
