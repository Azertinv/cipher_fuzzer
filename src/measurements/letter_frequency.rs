use crate::common::*;
use crate::measurements::{Measure, Summary};

pub fn measure(cts: &Cts) -> Measure {
    let mut count = [0; CT_ALPHABET_USIZE];
    let mut sum = 0;
    for ct in cts.iter() {
        sum += ct.len();
        for l in ct.iter() {
            count[*l as usize] += 1;
        }
    }
    let freq: [f64; CT_ALPHABET_USIZE] = count.iter().map(|x| {
        *x as f64 / sum as f64
    }).collect::<Vec<f64>>().try_into().unwrap();
    let summary = Summary::generate(&freq);
    Measure::LetterFrequency { freq, summary }
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
        let Measure::LetterFrequency{ freq, summary } = measure(&data);
        println!("{:?}", freq);
        println!("{:?}", summary);
        assert_eq!(freq[1], 2.0 / 9.0);
        assert_eq!(freq[2], 2.0 / 9.0);
        assert_eq!(freq[3], 2.0 / 9.0);
        assert_eq!(freq[4], 1.0 / 9.0);
        assert_eq!(freq[5], 1.0 / 9.0);
        assert_eq!(freq[6], 1.0 / 9.0);
        assert_eq!(summary.minimum, 0.0);
        assert_eq!(summary.maximum, 2.0/9.0);
    }
}
