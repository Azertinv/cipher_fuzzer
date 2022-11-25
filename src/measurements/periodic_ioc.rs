use crate::common::*;
use crate::measurements::{
    Measure,
    get_periodic_letter_count,
    get_ioc,
};

const MAX_PERIOD: usize = 50;

#[derive(Debug)]
pub struct PeriodicIoC {
    values: Vec<f64>,
}

impl Measure for PeriodicIoC {
    fn measure(cts: &Cts) -> Box<dyn Measure>  {
        let values: Vec<f64> = (1..MAX_PERIOD)
            .map(|period| get_ioc(get_periodic_letter_count(cts, period))).collect();
        Box::new(PeriodicIoC { values })
    }

    fn extract(&self) -> Vec<f64> {
       self.values.clone()
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
        assert_eq!(83.0, peak);
        assert_eq!(result.values[6], peak);
        assert_eq!(result.values[6+7], peak);
        assert_eq!(result.values[7+7], 0.0);
    }
}
