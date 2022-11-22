use crate::common::*;
use crate::measurements::Measure;

pub fn measure(cts: &Cts) -> Measure {
    let mut count = 0;
    for ct in cts.iter() {
        for i in 1..ct.len() {
            if ct[i] == ct[i - 1] {
                count += 1;
            }
        }
    }
    Measure::LetterRepeats { count }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn measurement() {
        let data: Cts = vec![
            vec![1, 1, 3],
            vec![1, 2, 3],
            vec![4, 5, 5],
        ];
        if let Measure::LetterRepeats{ count } = measure(&data) {
            println!("{:?}", count);
            assert_eq!(count, 2);
        } else {
            panic!();
        }
    }
}
