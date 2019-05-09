// Steps in Prime
// CodeWars challenge
// Program will accept two integers and should calculate the primes within
// the integers and from those primes identify a consecutive pair that
// shares the specified "step" between them.

use std::env;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();

    let step_num: Option<i32>;
    let first_num: Option<u64>;
    let second_num: Option<u64>;
    
    // Performing an action based on number of arguments passed
    match args.len() {
        0 | 1 => {
            eprintln!("You did not provide any arguments");
            help();
        },
        2 | 3 => {
            eprintln!("You did not provide enough arguments");
            help();
        },
        4 => {
           step_num= match args[2].parse() {
                Ok(num) => Some(num),
                Err(e) => {
                    eprintln!("Arg 1 error: {}", e);
                    help();
                    None
                }
            };
            first_num = match args[3].parse() {
                Ok(num) => Some(num),
                Err(e) => {
                    eprintln!("Arg 2 error: {}", e);
                    help();
                    None
                }
            };
            second_num = match args[4].parse() {
                Ok(num) => Some(num),
                Err(e) => {
                    eprintln!("Arg 3 error: {}", e);
                    help();
                    None
                }
            }
        },
        _ => {
            eprintln!("Incorrect arguments");
            help();
        }
    }

    // TODO pass g, m, and n to the step function
    // unwrap the option values

   if let Some(g) = step_num {
       // Assigned a value to step_numif it exists
   } else {
        eprintln!("Arg 1 error. Exiting");

   }
   if let Some(m) = first_num {

   } else {

   }
   if let Some(n) = second_num {

   } else {

   }
    println!("Hello, world!");
}


fn parse_input(args: Vec<String>) -> [Result<i32, ParseIntError>, Result<u64, ParseIntError>, Result<u64, ParseIntError>] {
    
}

fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut prime_container = Vec::new();
    
    'outer: for x in m..=n {
        let mut prime = false;
        'inner: for y in 2..x {
            if x % y == 0 {
                // if y is a factor of x, continue to next iteration
                // of outer loop
                continue 'outer;
            } else {
                // this will be set to true throughout this loop but
                // assuming the number is not a true prime, will be
                // set back to false once it encounters a factor of
                // x and reverts back to the outer loop
                prime = true;
            }
        }
        if prime {
            prime_container.push(x);
        }
    }

    // check for step between primes
    // check length of prime_container first
    let mut return_value: Option<(u64, u64)> = None;

    if prime_container.len() > 0 {
        for number in 0..prime_container.len() {
            if number == (prime_container.len() - 1) {
                return_value =  None;
            } else {
                let g_step = prime_container[number + 1] - prime_container[number];
                if (g_step as i32) == g {
                    return_value = Some((prime_container[number], prime_container[number + 1]));
                }
            }
        }
    } else {
        return_value = None;
    }
    return_value
}

fn help() {
    println!("Usage: steps_in_prime <step> <beginning of range> <end of range>");
}
