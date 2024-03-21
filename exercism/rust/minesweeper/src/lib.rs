pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();

    minefield
        .iter()
        .map(|row| row.as_bytes())
        .enumerate()
        .map(|(row_index, row)| {
            let lower_row = row_index.saturating_sub(1);
            let upper_row = row_index.saturating_add(2).min(height);

            let width = row.len();

            row.iter()
                .enumerate()
                .map(|(col_index, cell)| match cell {
                    b' ' => {
                        let lower_col = col_index.saturating_sub(1);
                        let upper_col = col_index.saturating_add(2).min(width);

                        let neighbor_mines_count = minefield[lower_row..upper_row]
                            .iter()
                            .flat_map(|row| row.as_bytes()[lower_col..upper_col].iter())
                            // use `.filter(_).count()` due to this optimization:
                            // https://doc.rust-lang.org/1.48.0/src/core/iter/adapters/mod.rs.html#1099-1118
                            .filter(|c| **c == b'*')
                            .count();

                        match neighbor_mines_count {
                            0 => ' ',
                            value => (value as u8 + b'0') as char,
                        }
                    }
                    &value => value as char,
                })
                .collect()
        })
        .collect()
}
