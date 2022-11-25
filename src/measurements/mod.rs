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
pub mod periodic_ioc;
pub use periodic_ioc::PeriodicIoC;

pub fn measure(cts: &Cts) -> Vec<f64>{
    let measure_fns = [
        LetterRepeats::measure,
        LetterFrequency::measure,
        IndexBounds::measure,
        IoC::measure,
        IsomorphsCounts::measure,
        PeriodicIoC::measure,
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

pub fn get_letter_count(cts: &Cts) -> Vec<i64> {
    get_periodic_letter_count(cts, 1)
}

pub fn get_periodic_letter_count(cts: &Cts, period: usize) -> Vec<i64> {
    let mut count = [0i64; CT_ALPHABET_USIZE];
    for ct in cts.iter() {
        for l in ct.iter().step_by(period) {
            count[*l as usize] += 1;
        }
    }
    count.to_vec()
}

pub fn get_ioc(count: Vec<i64>) -> f64 {
    let sum: i64 = count.iter().sum();
    CT_ALPHABET.iter()
        .map(|l| {
            let c: f64 = (count[*l as usize] * (count[*l as usize] - 1)) as f64;
            let n: f64 = (sum * (sum - 1)) as f64;
            c / n
        }).sum::<f64>() * (CT_ALPHABET_SIZE as f64)
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
