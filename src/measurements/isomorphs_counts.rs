use crate::common::*;
use crate::measurements::{
    Measure,
    Summary,
    get_isomorphs,
};

const MAX_ISOMORPH_SIZE: usize = 15;

#[derive(Debug)]
pub struct IsomorphsCounts {
    counts: Vec<u32>,
    summary: Summary<u32>,
}

impl Measure for IsomorphsCounts {
    fn measure(cts: &Cts) -> Box<dyn Measure>  {
        let cts_isomorphs: Vec<Vec<Vec<usize>>> = cts.iter()
            .map(|ct| get_isomorphs(ct, MAX_ISOMORPH_SIZE))
            .collect();
        let counts: Vec<u32> = (0..MAX_ISOMORPH_SIZE)
            .map(|i| cts_isomorphs.iter().map(|ct| ct[i].len() as u32).sum())
            .collect();
        let summary = Summary::generate(&counts);
        Box::new(IsomorphsCounts { counts, summary })
    }

    fn extract(&self) -> Vec<f64> {
        let mut result = self.summary.to_vec();
        result.append(&mut self.counts.iter().map(|v| *v as f64).collect());
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
            vec![1, 1, 3],
            vec![1, 2, 1],
            vec![4, 5, 5],
            vec![6, 1, 2, 3, 4, 5, 6],
        ];
        let result = IsomorphsCounts::measure(&data);
        let result: &IsomorphsCounts = result.as_any().downcast_ref().unwrap();
        println!("{:?}", result.counts);
        println!("{:?}", result.summary);
        assert_eq!(result.counts[0], 2);
        assert_eq!(result.counts[1], 1);
        assert_eq!(result.counts[2], 0);
        assert_eq!(result.counts[3], 0);
        assert_eq!(result.counts[4], 0);
        assert_eq!(result.counts[5], 1);
        assert_eq!(result.summary.minimum, 0);
        assert_eq!(result.summary.maximum, 2);
    }
}
