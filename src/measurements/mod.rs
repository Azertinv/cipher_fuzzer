use statistical as stat;
use crate::common::*;

pub mod letter_frequency;
pub use letter_frequency::LetterFrequency;
pub mod letter_repeats;
pub use letter_repeats::LetterRepeats;
pub mod index_bounds;
pub use index_bounds::IndexBounds;
pub mod index_of_coincidence;
pub use index_of_coincidence::IoC;
pub mod isomorphs_counts;
pub use isomorphs_counts::IsomorphsCounts;

pub fn measure(cts: &Cts) -> Vec<f64>{
    let measure_fns = [
        LetterRepeats::measure,
        LetterFrequency::measure,
        IndexBounds::measure,
        IoC::measure,
        IsomorphsCounts::measure,
    ];
    measure_fns.iter()
        .map(|measure_fn| measure_fn(cts))
        .flat_map(|measure| measure.extract())
        .collect()
}

pub fn get_isomorphs(ct: &[u8], max_size: usize) -> Vec<Vec<usize>> {
    let mut isomorphs = vec![Vec::new(); max_size];
    let ct_len = ct.len();
    for i in 1..=max_size {
        for j in i..ct_len {
            if ct[j - i] == ct[j] {
                isomorphs[i - 1].push(j - i);
            }
        }
    }
    isomorphs
}

pub fn get_letter_count_and_sum(cts: &Cts) -> (Vec<i64>, usize) {
    let mut count = [0i64; CT_ALPHABET_USIZE];
    let mut sum = 0;
    for ct in cts.iter() {
        sum += ct.len();
        for l in ct.iter() {
            count[*l as usize] += 1;
        }
    }
    (count.to_vec(), sum)
}

pub trait Measure : std::fmt::Debug {
    fn measure(cts: &Cts) -> Box<dyn Measure> where Self: Sized;

    fn extract(&self) -> Vec<f64>;

    fn as_any(&self) -> &dyn std::any::Any;
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

    fn to_vec(&self) -> Vec<f64>
    where
        T: Into<f64> + Copy,
    {
        vec![
            self.mean,
            self.median,
            self.minimum.into(),
            self.maximum.into(),
            self.stdev,
        ]
    }
}
