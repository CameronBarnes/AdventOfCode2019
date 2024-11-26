use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let row = 25;
    let col = 6;
    let layers = input
        .replace('\n', "")
        .chars()
        .chunks(row * col)
        .into_iter()
        .map(|chunk| chunk.collect_vec())
        .collect_vec();
    let layers = layers
        .into_iter()
        .map(|layer| {
            layer
                .into_iter()
                .chunks(row)
                .into_iter()
                .map(|test| test.collect_vec())
                .collect_vec()
        })
        .collect_vec();
    let mut out = vec![vec!['2'; row]; col];
    layers.iter().for_each(|layer| {
        for (row_num, row) in layer.iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                let pixel = &mut out[row_num][col_num];
                if *pixel == '2' {
                    *pixel = *col;
                }
            }
        }
    });
    let mut result = String::new();
    for row in out {
        result.push_str(
            &row.into_iter()
                .map(|pixel| if pixel == '1' { "■" } else { " " })
                .collect::<String>(),
        );
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() {
        // No good test to put here for this one but it is working
    }
}
