use array_const_fn_init::array_const_fn_init;

pub const CT_ALPHABET_SIZE: u8 = 83;
pub const CT_ALPHABET_USIZE: usize = CT_ALPHABET_SIZE as usize;

const fn usize_to_u8(i: usize) -> u8 {
    i as u8
}

pub const CT_ALPHABET: [u8; CT_ALPHABET_USIZE] = array_const_fn_init!(usize_to_u8; 83);

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
