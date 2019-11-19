//extern crate prime_ribs;
use ramp::int::Int;
use prime_ribs::new_prime;

fn main(){
    // Prime Generation
    let p = new_prime(512); // 512-bit prime
    let q = new_prime(64); // 64-bit prime
    println!("PRIMES:");
    println!("====================");
    println!("p: {}",p);
    println!();
    println!("q: {}",q);
    println!("====================");
}