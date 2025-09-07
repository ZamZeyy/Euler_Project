//find the largest prime factor of a given number

pub fn solve(number_to_analyse: u64){

    let mut prime_vectors: Vec<u64> = Vec::new();
    let mut prime_vectors_in_number: Vec<u64> =  Vec::new();
 
    for n in 1..number_to_analyse/2{

        

        let mut is_prime_product:bool = false;

        // need a faster prime finding algorithm
        for prime in prime_vectors.iter(){
            if *prime != 1 && n % prime == 0 {
                is_prime_product = true;
                break;
            }
        }

        if is_prime_product {
            continue;
        }
        
        let mut is_not_prime: bool = false;
        let mut multiples: Vec<u64> = Vec::new();
        for i in 1.. n{
            
            if i % n == 0{
                multiples.push(i);
            }

            if multiples.len() >= 2 {
                is_not_prime = true;
                break;
            }
        }
        
        if is_not_prime{
            continue;
        }
        println!("{n}");

        prime_vectors.push(n);

        if number_to_analyse % n == 0 {

            println!("prime product : {n}");
            prime_vectors_in_number.push(n);
        }
    }

    if prime_vectors_in_number.len() == 0{
            println!("{number_to_analyse} is a prime")
    }
    else{
        
        println!("{number_to_analyse} is a prime composite") 
    }
}