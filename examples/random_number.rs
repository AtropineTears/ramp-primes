extern crate prime_ribs;
use ramp::int::Int;
use prime_ribs::new_uint;

fn main(){
    // Random Large Unsigned Integer Generation
    let random_number = new_uint(1024);
    
    println!("RANDOM NUMBER:");
    println!("====================");
    println!("Number: {}",random_number);
    println!("====================");
}