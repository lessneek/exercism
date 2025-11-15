pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            // let (max_idx, &max_row_value) = row.iter().enumerate().max_by_key(|(_, &x)| x)?;
            let &max_row_value = row.iter().max()?;
            let max_row_values = row
                .iter()
                .enumerate()
                .filter(|&(_, &value)| value == max_row_value)
                .collect::<Vec<_>>();

            let result = max_row_values
                .iter()
                .filter_map(|&(max_idx, _)| {
                    let (min_in_column_row_idx, min_in_column_value) = input
                        .iter()
                        .enumerate()
                        .map(|(i, x)| (i, x[max_idx]))
                        .min_by_key(|(_, x)| *x)?;

                    if row_idx == min_in_column_row_idx || max_row_value == min_in_column_value {
                        Some((row_idx, max_idx))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            Some(result)
        })
        .flatten()
        .collect()
}
