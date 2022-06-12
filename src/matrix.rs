pub struct FMatrix{
    row_data: Box<[f64]>,
    column_data: Box<[f64]>,
    rows: usize,
    columns: usize,
}
impl FMatrix{
    pub fn new(columns: usize, rows: usize) -> Self {
        FMatrix{
            row_data: (vec![0.0; rows*columns]).into_boxed_slice(),
            column_data: (vec![0.0; rows*columns]).into_boxed_slice(),
            rows: rows,
            columns: columns
        }
    }

    pub fn get_val_from_rows(&self, col: usize, row: usize) -> f64 {
        return self.row_data[row + col * self.rows];
    }
    pub fn get_val_from_columns(&self, col: usize, row: usize) -> f64 {
        return self.column_data[col + row * self.columns]
    }

    pub fn set_val(&mut self, col: usize, row: usize, val: f64) {
        let row_data_idx = row + col * self.rows;
        let col_data_idx = col + row * self.columns;

        self.column_data[col_data_idx] = val;
        self.row_data[row_data_idx] = val;
    }

    pub fn to_string(&self) -> String {
        let capacity_required: usize = self.rows * self.columns * 3; // every entry has an accompanying ", " or "\n", so *3 is good enough
        let mut repr = String::with_capacity(capacity_required);

        let mut str_ptr = 0;

        for i in 0..self.rows {
            for j in 0..self.columns {
                repr += &self.get_val_from_columns(j, i).to_string();
                repr += ", ";
            }
            repr += "\n";
        }

        return repr;
    }
}