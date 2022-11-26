use crate::{
    common::*,
    measurements::measure,
    samples::random_ciphertexts,
};
use statistical as stat;
use std::iter::{zip, Zip};
use std::slice::Iter;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug)]
pub struct Sigmas {
    pub values: Vec<f64>,
}

impl Sigmas {
    pub fn distance(&self, rhs: &Sigmas) -> f64 {
        let mut total_distance = 0.0;
        for (lhs, rhs) in zip(&self.values, &rhs.values) {
            total_distance += (lhs - rhs).abs().exp2();
        }
        total_distance
    }
}

#[derive(Serialize, Deserialize)]
pub struct Distributions {
    means: Vec<f64>,
    stdevs: Vec<f64>,
}

impl Distributions {
    fn load_cache(name: &str) -> io::Result<Distributions> {
        let mut file = File::open(&format!("/tmp/dists_{name}.cache"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn save_cache(&self, name: &str) -> io::Result<()> {
        let mut file = File::create(&format!("/tmp/dists_{name}.cache"))?;
        let content = serde_json::to_string(self)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn generate(gen_fn: fn() -> Cts) -> Self {
        let mut results = vec![];
        for _ in 0..10000 {
            results.push(measure(&gen_fn()));
        }
        let mut means = vec![];
        let mut stdevs = vec![];
        let measures_size = results[0].len();
        for i in 0..measures_size {
            let values: Vec<f64> = results.iter().map(|x| x[i]).collect();
            means.push(stat::mean(&values));
            stdevs.push(stat::standard_deviation(&values, None));
        }
        Distributions { means, stdevs }
    }

    fn load_cache_or_generate(name: &str, gen_fn: fn() -> Cts) -> Self {
        println!("Loading {name} distributions from cache");
        match Self::load_cache(name) {
            Ok(dist) => return dist,
            Err(err) => {
                println!("Couldn't load distribution cache for \"{}\": {}", name, err);
            },
        }
        println!("Generating distributions");
        let dist = Self::generate(gen_fn);
        println!("Distributions generation done, saving to cache");
        if let Err(err) = dist.save_cache(name) {
            println!("Couldn't save distribution cache for \"{}\": {}", name, err);
        };
        dist
    }

    pub fn random() -> Self {
        Self::load_cache_or_generate("random", random_ciphertexts)
    }

    fn iter(&self) -> Zip<Iter<f64>, Iter<f64>> {
        zip(&self.means, &self.stdevs)
    }

    pub fn sigmas(&self, measures: &[f64]) -> Sigmas {
        let mut values = Vec::with_capacity(measures.len());
        for (measure, (mean, stdev)) in zip(measures, self.iter()) {
            if *stdev == 0.0 {
                values.push(0.0);
            } else {
                values.push(measure/stdev - mean/stdev);
            }
        }
        Sigmas { values }
    }
}
