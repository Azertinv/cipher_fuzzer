use crate::common::*;
use crate::measurements::{
    Measure,
    Summary,
    get_letter_count,
};

#[derive(Debug)]
pub struct LetterFrequency {
    summary: Summary<f64>,
}

impl Measure for LetterFrequency {
    fn measure(cts: &Cts) -> Box<dyn Measure>  {
        let count = get_letter_count(cts);
        let sum: i64 = count.iter().sum();
        let freq: [f64; CT_ALPHABET_USIZE] = count.iter().map(|x| {
            *x as f64 / sum as f64
        }).collect::<Vec<f64>>().try_into().unwrap();
        let summary = Summary::generate(&freq);
        Box::new(LetterFrequency { summary })
    }

    fn extract(&self) -> Vec<f64> {
       vec![
           self.summary.median,
           self.summary.minimum,
           self.summary.maximum,
           self.summary.stdev,
       ]
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
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let result = LetterFrequency::measure(&data);
        let result: &LetterFrequency = result.as_any().downcast_ref().unwrap();
        println!("{:?}", result.summary);
        assert_eq!(result.summary.minimum, 0.0);
        assert_eq!(result.summary.maximum, 2.0/9.0);
    }
}
