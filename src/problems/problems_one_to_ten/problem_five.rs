//2520  is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20 ?

use crate::problems::problems_one_to_ten::problem_three;

pub fn solve(max_multiple:u64){
    use std::time::Instant;
    let now = Instant::now();

    let mut primes :Vec<u64> = (1..=max_multiple).collect();
    primes.retain(|p| problem_three::is_prime(*p));

    let mut prime_powers :Vec<PrimePower> = Vec::new();

    for prime in primes{
        let mut max_pow: u32 = 0;
        let mut value: u64 = 0;
        while  value < max_multiple  {
            max_pow += 1;
            value = prime.pow(max_pow);
        }

        // we do not want the max_pow that breaks out the loop since itll be over max_multiple
        prime_powers.push(PrimePower { prime: prime, power: max_pow - 1 });
    }

    let mut solution : u128 = 1;

    for prime_power in prime_powers{
        solution *= prime_power.prime.pow(prime_power.power) as u128;
    }

    println!("solution is {solution}");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); //104.20µs
}


struct PrimePower {
    prime: u64,
    power: u32,
}