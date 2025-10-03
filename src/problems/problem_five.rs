//2520  is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20 ?

pub fn solve(max_multiple:i64){
    use std::time::Instant;
    let now = Instant::now();
    let mut multiple_hit :bool = false;
    let mut count: i64 = 1;

    while multiple_hit == false{
        
        for multiple in (1..max_multiple).rev(){
            if count % multiple != 0  {
                break;
            }
            if multiple == 1{
                println!("{count} Hit!");
                multiple_hit = true;
            }
        }
        count += 1;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}