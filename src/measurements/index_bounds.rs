use crate::common::*;
use crate::measurements::{Measure, Summary};

#[derive(Debug)]
pub struct IndexBounds {
    summary: Summary<u8>,
}

impl Measure for IndexBounds {
    fn measure(cts: &Cts) -> Box<dyn Measure> {
        if cts.len() < 2 {
            return Box::new(IndexBounds { summary: Summary::generate(&[]) })
        }
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
        Box::new(IndexBounds { summary: Summary::generate(&bounds) })
    }

    fn extract(&self) -> Vec<f64> {
        self.summary.to_vec()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
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
        let result = IndexBounds::measure(&data);
        let result: &IndexBounds = result.as_any().downcast_ref().unwrap();
        assert_eq!(result.summary.minimum, 49);
        assert_eq!(result.summary.maximum, 82);
    }
}
