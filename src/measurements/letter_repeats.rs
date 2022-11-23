use crate::common::*;
use crate::measurements::Measure;

#[derive(Debug)]
pub struct LetterRepeats {
    count: usize,
}

impl Measure for LetterRepeats {
    fn measure(cts: &Cts) -> Box<dyn Measure> {
        let mut count = 0;
        for ct in cts.iter() {
            for i in 1..ct.len() {
                if ct[i] == ct[i - 1] {
                    count += 1;
                }
            }
        }
        Box::new(LetterRepeats { count })
    }

    fn extract(&self) -> Vec<f64> {
        vec![self.count as f64]
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
            vec![1, 1, 3],
            vec![1, 2, 3],
            vec![4, 5, 5],
        ];
        let result = LetterRepeats::measure(&data);
        let result: &LetterRepeats = result.as_any().downcast_ref().unwrap();
        println!("{:?}", result.count);
        assert_eq!(result.count, 2);
    }
}
