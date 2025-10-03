//A palindromic number reads the same both ways. The largest palindrome made from the product of two -digit numbers is 
//9009 = 91 x 99
// Find the largest palindrome made from the product of two 3-digit numbers.

use std::{num::NonZero};

pub fn solve(didgets: Option<NonZero<u32>>){

    let didgets_value = match didgets{
        Some(value) => value,
        None => {println!("Didgets are invalid"); return;}
    };
    let max_num = 10_i32.pow(didgets_value.into())-1; 
    let min_num = 10_i32.pow((didgets_value.get() - 1).into()); 

    println!("Max Number {}", max_num);
    println!("Min Number{}", min_num);
    
    let mut highest :i128 = 0;

    for lhs in (min_num..=max_num).rev(){
        for rhs in (min_num..=max_num).rev(){
            let product:i128= (lhs as i128 * rhs as i128); // avoid overflow of large multiplications
            let product_str:String = product.to_string();
            let split_point: usize = product_str.len() / 2; 
            
            let (lhs_split, rhs_split): (&str, &str) = product_str.split_at(split_point);
            let reverse_rhs : String = rhs_split.chars().rev().collect();
            
            if(lhs_split != reverse_rhs){
                continue;
            }
                println!("Found one {lhs} x {rhs} = {lhs_split} {rhs_split}");
                
                if (highest > product){
                    continue;
                }
                highest = product;
                println!("Newest highest {highest}");
        }
    }
    println!("Highest Palindrome = {highest}")
}