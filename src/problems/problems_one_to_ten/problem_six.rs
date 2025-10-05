pub fn solve(required_numbers: u32){
    
    let natrual_numbers : Vec<u32> = (1..=required_numbers).collect();

    let mut sum_of_squares: u32 = 0;
    let mut sum :u32 = 0;
    for number in natrual_numbers{
        sum_of_squares += number.pow(2);
        sum += number;
    }

    let sum_squared: u32 = sum.pow(2) ;

    println!("sum of squares: {sum_of_squares}");
    println!("sum squared: {sum_squared}");
    
    let sum_square_diffrance = sum_squared - sum_of_squares;

    println!("sum of squares: {sum_square_diffrance}");
}