pub struct PascalsTriangle {
    pub rows: u32,
    //pt: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row: u32) -> Self {
        PascalsTriangle {
            rows: row
        }
        /*PascalsTriangle {
            pt: Vec::with_capacity(row),
        }*/

    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut pt: Vec<Vec<u32>> = Vec::new();
        let mut curr_row: Vec<u32>;

        if self.rows == 0 {
            return pt;
        }
        pt.push(vec![1]);
        if self.rows == 1 {
            return pt;
        }
        curr_row = vec![1];
        for _ in 1 .. self.rows {
            curr_row = next_row(&curr_row);
            pt.push(curr_row.clone());
        }
        pt
    }
}

pub fn next_row(vec: &Vec<u32>) -> Vec<u32> {
    let zero = [0];
    let row1 = zero.iter().chain(vec.iter());
    let row2 = vec.iter().chain(zero.iter());
    row1.zip(row2).map(|(a, b)| a + b).collect()
}
