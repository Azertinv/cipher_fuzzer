pub const CT_ALPHABET_SIZE: u8 = 83;
pub const CT_ALPHABET_USIZE: usize = CT_ALPHABET_SIZE as usize;

pub const CT_ALPHABET: [u8; CT_ALPHABET_USIZE] = [
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
    70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82,
];

pub const READABLE_OFFSET: u8 = 32;

pub const CT_PER_CTS: usize = 9;

pub const MAX_CT_USIZE: usize = 138;
pub const MIN_CT_USIZE: usize = 99;

pub type Ct = Vec<u8>;
pub type Cts = Vec<Ct>;

pub fn print_texts(cts: &Cts) {
    for ct in cts.iter() {
        let ct = String::from_utf8(ct.iter().map(|x| x + READABLE_OFFSET).collect()).unwrap();
        println!("{}", ct);
    }
}

pub fn get_columns(cts: &Cts) -> Vec<Vec<u8>> {
    let mut result = vec![];
    let max_ct_len = cts.iter().map(Vec::len).max().unwrap();
    for i in 0..max_ct_len {
        let mut index = vec![];
        for ct in cts.iter() {
            if i < ct.len() {
                index.push(ct[i]);
            }
        }
        result.push(index);
    }
    result
}

/// Helper function for `Cipher`s
pub fn substitute(data: &mut [u8], alphabet: &[u8]) {
    assert!(alphabet.len() == CT_ALPHABET_SIZE as usize);
    for elem in data.iter_mut() {
        *elem = *alphabet.get(*elem as usize)
            .expect("substitute() letter in data is out of bound");
    }
}
