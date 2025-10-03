use std::num::NonZero;

mod problems;

fn main() {

    const DIDGETS:Option<NonZero<u32>>  = NonZero::<u32>::new(4);
    problems::problem_four::solve( DIDGETS);
    
}