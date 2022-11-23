use crate::common::*;
use crate::measurements::{Measure, Summary};

pub fn measure(cts: &Cts) -> Measure {
    let mut columns = get_columns(cts);
    let mut holes_in_columns: Vec<Vec<u8>> = vec![];
    for column in columns.iter_mut() {
        column.sort();
        let mut holes = vec![];
        let column_len = column.len();
        for i in 0..column_len {
            let first = column[i] as i32;
            let second = column[(i+1) % column_len] as i32;
            if first == second { continue }
            holes.push((second - first).rem_euclid(CT_ALPHABET_SIZE.into()) as u8);
        }
        if !holes.is_empty() {
            holes_in_columns.push(holes);
        }
    }
    let bounds: Vec<u8> = holes_in_columns.iter()
        .map(|holes| *holes.iter().max().unwrap())
        .collect();
    Measure::IndexBounds { summary: Summary::generate(&bounds) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn measurement() {
        let data: Cts = vec![
            "AAA   BBB".as_bytes().to_vec(),
            "A BA BA B".as_bytes().to_vec(),
        ];
        if let Measure::IndexBounds{ summary } = measure(&data) {
            assert_eq!(summary.minimum, 49);
            assert_eq!(summary.maximum, 82);
        } else {
            panic!();
        }
    }
}
