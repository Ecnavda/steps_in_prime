// Steps in Prime
// CodeWars challenge
// Program will accept two integers and should calculate the primes within
// the integers and from those primes identify a consecutive pair that
// shares the specified "step" between them.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let g: i32;
    let m: u64;
    let n: u64;
    
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
            g = match args[2].parse() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("{}", e);
                    help();
                }
            }  

            m = match args[3].parse() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("{}", e);
                    help();
                }
            }  

            n = match args[4].parse() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("{}", e);
                    help();
                }
            }  
        }
    }

    //TODO accept stdin parameters for g, m, and n
    println!("Hello, world!");
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
