use ramp_primes::new_prime;

fn main(){
    // Prime Generation
    let p = new_prime(512); // 512-bit prime
    let q = new_prime(64); // 64-bit prime
    
    // Print out prime numbers
    println!("PRIMES:");
    println!("p: {}",p);
    println!();
    println!("q: {}",q);
}