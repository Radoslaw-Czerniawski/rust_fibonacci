use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use tramp::{rec_call, rec_ret, tramp, Rec};

fn main() {
    let nth_element: usize = prompt_for_nth_fibonacci();

    println!(
        "{nth_element} element of fibonacci sequence is {}",
        fibonacci(nth_element, Zero::zero(), One::one())
    )
}

fn prompt_for_nth_fibonacci() -> usize {
    println!("Please input nth position of fibonacci sequence you desire.");

    loop {
        let mut nth_element = String::new();

        io::stdin()
            .read_line(&mut nth_element)
            .expect("Please enter correct desired nth position of fibonacci sequence");

        let nth_element: usize = match nth_element.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input whole number nth position of fibonacci sequence!");
                continue;
            }
        };

        break nth_element;
    }
}

fn fibonacci(n: usize, accumulator: BigUint, previous: BigUint) -> BigUint {
    tramp(tail_opt_fibonacci(n, accumulator, previous))
}

fn tail_opt_fibonacci(n: usize, accumulator: BigUint, previous: BigUint) -> Rec<BigUint> {
    if n <= 0 {
        rec_ret!(accumulator)
    } else {
        rec_call!(tail_opt_fibonacci(
            n - 1,
            &accumulator + previous,
            accumulator
        ))
    }
}
