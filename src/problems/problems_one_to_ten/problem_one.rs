pub fn solve(sum_to: i32 ) -> i32 {
    let mut sum: i32 = 0;
    for n in 1..sum_to{
        if !(n % 3 == 0 || n % 5 == 0){continue;}
        sum += n;
    }
    return sum;
}