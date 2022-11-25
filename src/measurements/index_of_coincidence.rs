use crate::common::*;
use crate::measurements::{
    Measure,
    get_letter_count,
    get_ioc,
};

#[derive(Debug)]
pub struct IoC {
    value: f64,
}

impl Measure for IoC {
    fn measure(cts: &Cts) -> Box<dyn Measure>  {
        let value = get_ioc(get_letter_count(cts));
        Box::new(IoC { value })
    }

    fn extract(&self) -> Vec<f64> {
       vec![self.value]
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
            vec![4, 5, 6],
            vec![7, 9, 9],
        ];
        let result = IoC::measure(&data);
        let result: &IoC = result.as_any().downcast_ref().unwrap();
        println!("{result:?}");
        assert!((result.value - 0.027777777777777 * 83.0).abs() < 0.0000000000001);
        assert_eq!(result.value, result.extract()[0]);
    }
}
