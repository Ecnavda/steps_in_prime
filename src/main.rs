// Steps in Prime
// CodeWars challenge
// Program will accept two integers and should calculate the primes within
// the integers and from those primes identify a consecutive pair that
// shares the specified "step" between them.


fn main() {
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
    if prime_container.len > 0 {

    }

    //TODO compare primes from prime_container for the specified
    // step value
    // REMINDER: 0..prime_container.len, prime_container[x], prime_container[x + 1]

}
