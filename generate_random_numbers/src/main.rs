extern crate rand_distr;
extern crate rand;

use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform, Standard, Alphanumeric};
use rand_distr::{Distribution as Distr, Normal, NormalError};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn generate_random_password() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}

fn generate_random_numbers_of_custom_type() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

// fn generate_random_numbers_in_distribution() -> Result<(), NormalError> {
//     let mut rng = rand::thread_rng();
//     let normal = Normal::new(2.0, 3.0)?;
//     let v = normal.sample(&mut rng);
//     println!("{} is from a N(2.9) distribution", v);
//     return Ok(())
// }

// Generates random numbers within an uniform range
fn generate_random_numbers_in_range_uniform() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

// Generates random numbers within a range
fn generate_random_numbers_in_range() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}

// Generates random numbers
fn generate_random_numbers() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn main() {
    generate_random_numbers();
    generate_random_numbers_in_range();
    generate_random_numbers_in_range_uniform();
    generate_random_numbers_of_custom_type();
    generate_random_password();
    // generate_random_numbers_in_distribution();
}
