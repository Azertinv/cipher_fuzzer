use crate::common::*;

pub fn delta_stream(cts: &Cts) -> Cts {
    cts.iter().map(|ct| {
        (1..ct.len()).map(|i| {
            (CT_ALPHABET_SIZE + ct[i] - ct[i - 1]).rem_euclid(CT_ALPHABET_SIZE)
        }).collect()
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn delta_stream() {
        let data: Cts = vec![
            vec![0,0,0,13,13,13,1,1,1],
            vec![0,13,1,0,13,1,0,13,1],
        ];
        let result = super::delta_stream(&data);
        assert_eq!(result[0], vec![0, 0, 13, 0, 0, 14, 0, 0]);
        assert_eq!(result[1], vec![13, 14, 25, 13, 14, 25, 13, 14]);
    }
}
