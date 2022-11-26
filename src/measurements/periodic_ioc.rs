use crate::common::*;
use crate::measurements::{
    Measure,
    Summary,
    get_periodic_letter_count,
    get_ioc,
};

const MAX_PERIOD: usize = 30;

#[derive(Debug)]
pub struct PeriodicIoC {
    values: Vec<f64>,
    summary: Summary<f64>,
}

impl Measure for PeriodicIoC {
    fn measure(cts: &Cts) -> Box<dyn Measure>  {
        let values: Vec<f64> = (1..MAX_PERIOD)
            .map(|period| get_ioc(get_periodic_letter_count(cts, period))).collect();
        let summary = Summary::generate(&values);
        Box::new(PeriodicIoC { values, summary })
    }

    fn extract(&self) -> Vec<f64> {
       let mut result = self.values.clone();
       result.append(&mut self.summary.to_vec());
       result
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
            vec![
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
                0, 1, 2, 3, 4, 5, 6,
            ],
        ];
        let result = PeriodicIoC::measure(&data);
        let result: &PeriodicIoC = result.as_any().downcast_ref().unwrap();
        println!("{result:?}");
        let peak = *result.values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        assert_eq!(CT_ALPHABET_SIZE as f64, peak);
        assert_eq!(result.values[6], peak);
        assert_eq!(result.values[6+7], peak);
        assert_eq!(result.values[7+7], 0.0);
    }
}
