pub fn solve(max_sequence_element: u32, mut n_minus_one: u32, mut n_minus_two:u32) -> Result<u32,&'static str> {
    let mut sum: u32 = 0;
    let mut current_sequence_value: u32;

    if n_minus_one + n_minus_two > max_sequence_element { return Err("Max sequence is smaller then the product of a and b "); }
    if n_minus_one + n_minus_two == 0 { return Err("cannot create a sequence when the first two elements equal zero "); }

    if n_minus_one % 2 == 0 {sum+= n_minus_one;}
    if n_minus_two % 2 == 0 {sum+= n_minus_two;}

    while n_minus_one + n_minus_two < max_sequence_element {
        current_sequence_value = n_minus_one + n_minus_two;
        n_minus_two = n_minus_one;
        n_minus_one = current_sequence_value;
        
        if !(current_sequence_value % 2 == 0) { continue; }

        match sum.checked_add(current_sequence_value){
            Some(added ) => sum = added,
            None => return Err("Overflow on sum")
        };
    }

    return Ok(sum);
}