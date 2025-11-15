pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count);

        for n in 0..row_count {
            let mut row = Vec::with_capacity(n + 1);

            if let Some(prev_row) = rows.last() {
                for i in 0..=n {
                    let a = if i == 0 { 0 } else { prev_row[i - 1] };
                    let b = if i > prev_row.len() - 1 {
                        0
                    } else {
                        prev_row[i]
                    };
                    row.push(a + b);
                }
            } else {
                row.push(1);
            }

            rows.push(row);
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
