use std::cell::RefCell;

const VARIABLES_NUMBER_COLUMN: usize = 3;
const ROWS_NUMBER_COLUMN: usize = 4;
const LABELS_NUMBER_COLUMN: usize = 5;

#[derive(Debug, Default)]
pub struct DataCompare {
    base: RefCell<Vec<String>>,
    compare: RefCell<Vec<String>>,
}

impl DataCompare {
    pub fn new() -> DataCompare {
        DataCompare::default()
    }
    pub fn set_base(&self, content: &str) {
        *self.base.borrow_mut() = content
            .split_ascii_whitespace()
            .into_iter()
            .map(|f| String::from(f))
            .collect::<Vec<_>>();
    }
    pub fn set_compare(&self, content: &str) {
        *self.compare.borrow_mut() = content
            .split_ascii_whitespace()
            .into_iter()
            .map(|f| String::from(f))
            .collect::<Vec<_>>();
    }
    pub fn equal(&self) -> bool {
        // variable number
        if !self.compare(VARIABLES_NUMBER_COLUMN) {
            return false;
        };
        // row number
        if !self.compare(ROWS_NUMBER_COLUMN) {
            return false;
        };
        // row number
        self.compare(LABELS_NUMBER_COLUMN)
    }

    fn compare(&self, column_number: usize) -> bool {
        let base = self.base.borrow();
        let compare = self.compare.borrow();
        let base = base.get(column_number);
        let compare = compare.get(column_number);
        base.eq(&compare)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compare_test() {
        let base = "SDTM.AE       15JUN23:13:26:12  15JUN23:13:26:12    35     960  \\u19981;\\u33391;\\u20107;\\u20214;";
        let compare = "SDTM_QC.V_AE  15JUN23:13:35:09  15JUN23:13:35:09    35     960  \\u19981;\\u33391;\\u20107;\\u20214;";
        let c = DataCompare::new();
        c.set_base(base);
        c.set_compare(compare);
        assert!(c.equal());

        let base = "SDTM.AE       15JUN23:13:26:12  15JUN23:13:26:12    35     961  \\u19981;\\u33391;\\u20107;\\u20214;";
        let compare = "SDTM_QC.V_AE  15JUN23:13:35:09  15JUN23:13:35:09    35     960  \\u19981;\\u33391;\\u20107;\\u20214;";
        let c = DataCompare::new();
        c.set_base(base);
        c.set_compare(compare);
        assert!(!c.equal());

        let base = "SDTM.AE       15JUN23:13:26:12  15JUN23:13:26:12    35     960";
        let compare = "SDTM_QC.V_AE  15JUN23:13:35:09  15JUN23:13:35:09    35     960  \\u19981;\\u33391;\\u20107;\\u20214;";
        let c = DataCompare::new();
        c.set_base(base);
        c.set_compare(compare);
        assert!(!c.equal());

        let base = "SDTM.AE       15JUN23:13:26:12  15JUN23:13:26:12    35     960";
        let compare = "SDTM_QC.V_AE  15JUN23:13:35:09  15JUN23:13:35:09    35     960";
        let c = DataCompare::new();
        c.set_base(base);
        c.set_compare(compare);
        assert!(c.equal());
    }
}
