use statistical as stat;
use crate::common::*;

pub mod letter_frequency;
pub mod letter_repeats;

pub fn measure(cts: &Cts) -> Vec<f64>{
    measures_to_values(&[
        letter_frequency::measure(cts),
        letter_repeats::measure(cts),
    ])
}

fn measures_to_values(measures: &[Measure]) -> Vec<f64> {
    measures.iter().flat_map(|m| m.extract()).collect()
}

#[derive(Debug)]
pub enum Measure {
    LetterFrequency {
        freq: [f64; CT_ALPHABET_USIZE],
        summary: Summary<f64>
    },
    LetterRepeats {
        count: usize,
    },
}

impl Measure {
    pub fn extract(&self) -> Vec<f64> {
        match self {
            Measure::LetterFrequency{ freq: _ , summary } => {
                vec![summary.median, summary.minimum, summary.maximum, summary.stdev]
            },
            Measure::LetterRepeats{ count } => {
                vec![*count as f64]
            },
        }
    }
}

#[derive(Debug)]
pub struct Summary<T> {
    mean: f64,
    median: f64,
    minimum: T,
    maximum: T,
    stdev: f64,
}

impl<T> Summary<T> {
    fn generate(data: &[T]) -> Self
    where
        T: PartialOrd + Copy,
        f64: From<T>,
    {
        let float_data: Vec<f64> = data.iter().map(|x| f64::from(*x)).collect();
        let mean = stat::mean(&float_data);
        let median = stat::median(&float_data);
        let minimum = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let maximum = *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let stdev = stat::standard_deviation(&float_data, None);
        Summary { mean, median, minimum, maximum, stdev }
    }
}
