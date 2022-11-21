use cipher_fuzzer::{
    samples::{
        get_texts,
        messages_vec,
    },
    distribution::{
        Distributions,
    },
    measurements::{
        measure,
    },
};

fn main() -> std::io::Result<()> {
    let random_dist = Distributions::random();

    let messages_p = random_dist.p_values(&measure(&messages_vec()));
    let input_p = random_dist.p_values(&measure(&get_texts()?));

    let score = messages_p.distance(&input_p);
    println!("{score}");

    Ok(())
}
