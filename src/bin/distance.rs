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

    let msg_measure = measure(&messages_vec());
    println!("{msg_measure:?}");
    let msg_p = random_dist.sigmas(&msg_measure);
    println!("{msg_p:?}");
    let input_measure = measure(&get_texts()?);
    println!("{input_measure:?}");
    let input_p = random_dist.sigmas(&input_measure);
    println!("{input_p:?}");

    let score = msg_p.distance(&input_p);
    println!("{score}");

    Ok(())
}
