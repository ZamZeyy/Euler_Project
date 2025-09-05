mod problem_one;
mod  problem_two;
fn main() {
    // let _problem_one: i32 = problem_one::solve(1000);
    // println!("Problem 1 solution = {_problem_one}");

    let _problem_two = match problem_two::solve(4000000, 2,1) {
        Ok(_problem_two) => println!("{_problem_two}"),
        Err(e) => println!("{}", e)
    };
    
}


