use ramp_primes::new_uint;

fn main(){
    // Random Large Unsigned Integer Generation
    let random_number = new_uint(1024);
    
    // Prints Out Random Number
    println!("Number: {}",random_number);
}