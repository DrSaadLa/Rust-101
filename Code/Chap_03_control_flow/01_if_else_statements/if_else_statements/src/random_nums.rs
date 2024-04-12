#[allow(unused_imports)]
use {
    rand::Rng,
    rand::SeedableRng,
    rand::rngs::StdRng,
    std::any::Any,
};


// Vector elements data type
pub enum NumberType {
    Integer,
    Float,
}

// The vector types: Vec<i64> or Vec<f64>
#[derive(Debug)]
pub enum NumberVec {
    IntegerVec(Vec<i64>),
    FloatVec(Vec<f64>),
}

pub fn gen_rand_nums(n: usize, min: i64, max: i64, number_type: NumberType, seed: Option<u64>) -> NumberVec {
    // let mut rng = rand::thread_rng();
    let mut rng = match seed {
        Some(seed_val) => StdRng::seed_from_u64(seed_val),
        None => StdRng::seed_from_u64(2), // Default seed value
    };


    match number_type {
        NumberType::Integer => {
            let mut results: Vec<i64> = Vec::with_capacity(n);
            for _ in 0..n {
                let value = rng.gen_range(min..=max); // Generates a random integer
                results.push(value);
            }
            NumberVec::IntegerVec(results)
        },
        NumberType::Float => {
            let mut results: Vec<f64> = Vec::with_capacity(n);
            for _ in 0..n {
                let value = rng.gen_range(min as f64..=max as f64); // Generates a random floating-point number
                results.push(value);
            }
            NumberVec::FloatVec(results)
        },
    }
}


// function that generates random numbers of print them in a formatted way.
pub fn print_gen_nums(number_type: NumberType, seed: Option<u64>) {
    let nums = gen_rand_nums(10, 1, 100, number_type, seed);

    match nums {
        NumberVec::IntegerVec(int_vec) => println!("Integer Vector: {:?}", int_vec),
        NumberVec::FloatVec(float_vec) => {
            println!("Float Vector: [{}]", float_vec.iter()
                                                    .map(|f| format!("{:.5}", f))
                                                    .collect::<Vec<_>>()
                                                    .join(", "));
        },
    }
}
