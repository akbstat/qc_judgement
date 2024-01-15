use crate::compare::DataCompare;
use anyhow::Result;
use std::{fs, path::Path};

const SECTD: &[u8] = r"\sectd".as_bytes();
const PARD: &[u8] = r"\pard\plain".as_bytes();
const LEFT_CURLY_BRACE: u8 = 123;
const RIGHT_CURLY_BRACE: u8 = 125;
const CELL: &str = r"\cell";
const DATA_SUMMARY: &str = "\\u25968;\\u25454;\\u38598;\\u27719;\\u24635;";
const VARIABLE_SUMMARY: &str = "\\u21464;\\u37327;\\u27719;\\u24635;";
const ROW_SUMMARY: &str = "\\u35266;\\u27979;\\u27719;\\u24635;";
const DIFFERENT_ATTRIBUTE: &str =
    "\\u20855;\\u26377;\\u19981;\\u21516;\\u23646;\\u24615;\\u30340;\\u21464;\\u37327;\\u25968;";
const UNEQAUL_ROW_NUMBER: &str = "\\u37096;\\u20998;\\u27604;\\u36739;\\u21464;\\u37327;\\u19981;\\u31561;\\u30340;\\u35266;\\u27979;\\u25968;";
const ZERO: &str = r"0\u12290;";

#[derive(Debug, PartialEq)]
enum Status {
    PardStart,
    CellStart,
    CellEnd,
}

#[derive(Debug, PartialEq)]
enum Field {
    DataSummary,
    VariableSummary,
    RawSummary,
    None,
}

pub struct QcJudge {
    contents: Vec<String>,
}

impl QcJudge {
    pub fn new(path: &Path) -> Result<QcJudge> {
        let qc = fs::read(path)?;
        let content_start = content_start_index(&qc).unwrap();
        Ok(QcJudge {
            contents: fetch_contents(&qc[content_start..]),
        })
    }
    pub fn judge(&self) -> bool {
        let mut field = Field::None;
        let mut i = 0;
        let data_compare = DataCompare::new();
        while i < self.contents.len() {
            match field {
                Field::None => {
                    if self.contents[i].eq(&DATA_SUMMARY) {
                        field = Field::DataSummary;
                    }
                }
                Field::DataSummary => {
                    if self.contents[i].eq(&VARIABLE_SUMMARY) {
                        field = Field::VariableSummary;
                        continue;
                    }
                    data_compare.set_base(self.contents.get(i + 1).unwrap());
                    data_compare.set_compare(self.contents.get(i + 2).unwrap());
                    if !data_compare.equal() {
                        return false;
                    }
                    i += 2;
                }
                Field::VariableSummary => {
                    if self.contents[i].eq(&ROW_SUMMARY) {
                        field = Field::RawSummary;
                        continue;
                    }
                    if self.contents[i].starts_with(&DIFFERENT_ATTRIBUTE) {
                        return false;
                    }
                }
                Field::RawSummary => {
                    if self.contents[i].starts_with(&UNEQAUL_ROW_NUMBER) {
                        let contents = self.contents[i]
                            .split_ascii_whitespace()
                            .into_iter()
                            .collect::<Vec<&str>>();
                        return contents.get(1).unwrap().eq(&ZERO);
                    }
                }
            }
            i += 1;
        }

        true
    }
}

/// fetch contents from rtf, store by rows
fn fetch_contents(data: &[u8]) -> Vec<String> {
    let mut contents = vec![];
    let mut status = Status::CellEnd;
    let mut cell_start = 0;
    for (i, c) in data.iter().enumerate() {
        if i < PARD.len() {
            continue;
        }
        match status {
            Status::PardStart => {
                if c.eq(&LEFT_CURLY_BRACE) {
                    cell_start = i + 1;
                    status = Status::CellStart;
                }
            }
            Status::CellStart => {
                if c.eq(&RIGHT_CURLY_BRACE) {
                    let content: String = String::from_utf8(data[cell_start..i].to_vec())
                        .unwrap()
                        .replace(CELL, "")
                        .trim()
                        .into();
                    if content.len() > 0 {
                        contents.push(content);
                    }
                    status = Status::CellEnd;
                }
            }
            Status::CellEnd => {
                if PARD.eq(&data[i - PARD.len() + 1..i + 1]) {
                    status = Status::PardStart;
                }
            }
        }
    }
    contents
}

/// find out the start index of content in rtf
fn content_start_index(data: &[u8]) -> Option<usize> {
    for (i, _) in data.iter().enumerate() {
        if i < SECTD.len() {
            continue;
        }
        let start = i - SECTD.len() + 1;
        if SECTD.eq(&data[start..i + 1]) {
            return Some(start);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_qc_test() {
        let path = Path::new(r"D:\Studies\ak112\303\stats\CSR\validation\qc-result\sdtm\v_ae.rtf");
        let judge = QcJudge::new(path).unwrap();
        assert!(judge.judge());

        let path = Path::new(r"D:\Studies\ak112\303\stats\CSR\validation\qc-result\sdtm\v_dd2.rtf");
        let judge = QcJudge::new(path).unwrap();
        assert!(!judge.judge());

        let path = Path::new(r"D:\Studies\ak112\303\stats\CSR\validation\qc-result\sdtm\V_cv.rtf");
        let judge = QcJudge::new(path).unwrap();
        assert!(!judge.judge());
    }

    #[test]
    fn content_start_index_test() {
        let path = Path::new(r"D:\Studies\ak112\303\stats\CSR\validation\qc-result\sdtm\V_cv.rtf");
        assert_ne!(content_start_index(&fs::read(path).unwrap()), None)
    }
}
