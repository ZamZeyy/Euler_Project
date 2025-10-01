
//find the largest prime factor of a given number

pub fn solve(number_to_analyse: u64)
{
    use std::time::Instant;
    let now = Instant::now();
    let mut prime_vectors: Vec<u64> = Vec::new();

    let primes_in_num = sieve_of_eratosthenes(number_to_analyse);
 
    for n in primes_in_num{
        if(number_to_analyse % n != 0){
            continue;
        }

        prime_vectors.push(n);

    }
    
    println!("{:?}", prime_vectors);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

pub fn is_prime(number: u64) -> bool
{
    let sqrt_number: u64 = f64::sqrt(number as f64).floor() as u64;

    for divisor in 2..=sqrt_number{
        if number % divisor == 0{
            return false;
        }
    }
    return true;
}

pub fn sieve_of_eratosthenes(number: u64) -> Vec<u64>{

    let sqrt_number: u64 = f64::sqrt(number as f64).floor() as u64;
    let mut primes: Vec<u64> = Vec::new();
    let mut possible_numbers: Vec<u64> = (2..sqrt_number).collect();  

    for divisor in 2..sqrt_number{
        if is_prime(divisor){
            possible_numbers.retain_mut(|x: &mut u64| *x % divisor != 0);
            let len = possible_numbers.len();
            println!("{len} Available numbers");
            primes.push(divisor);
        }
    }

    println!("{:?}", primes);
    return primes;

}