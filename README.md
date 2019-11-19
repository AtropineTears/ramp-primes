# Large Composite and Prime Number Generator

This crate provides a **beautifully simplistic API** for generating large, cryptographically-random, integers in rust, including but not limited to **prime numbers**.

It takes full advantage of the [RAMP](https://crates.io/crates/ramp) crate, which provides high-performance large integers through in-line assembly and a high level, simplistic interface for users to use.

## Usage

Add the following to your `cargo.toml`:

`prime_generator = "*"`

`ramp = "0.5"`

### Example of Prime Generation

```rust
use ramp::Int;
use prime_generator;

fn main(){
  // Generates two primes (p,q) both of 512 bits
  let p = new_prime(512);
  let q = new_prime(512);
}
```

## Prime Number Generation Design

The Prime Number Generation and parts of its code is based on [Snips-AI's Medium Blog on Generating Prime Numbers](https://medium.com/snips-ai/prime-number-generation-2a02f28508ff).

A **conservative attempt** is made at deciding whether a number is prime or not. The number goes through the generation phase and 3 tests to determine if its prime:

### Generation Phase

1. A Single Parameter is passed to the generator function that indicates the number of bits the prime should be.

2. The Userspace CSPRNG is seeded by the operating system to generate the random numbers using the rand crate.

3. An Unsigned Integer is generated until it passes the prime test, and before being sent to the test, the LSB is changed to 0 to indicate its odd and its MSB is changed to 1.

4. The number is sent to be processed by three tests

### Primality Tests

The numbers go through multiple tests to determine whether they are composite or prime.

1. An array of the first 2048 primes is used to check whether the number is divisble by any of the primes in the array.

2. **Fermat's Little Theorem** is performed

3. **Miller-Rabin Primality Test**, the gold standard recommended by the official RSA documentation and by [NIST](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf) on generating primes, is performed with 16 iterations, the same used by Apple's cryptosystem.

If the number passes these tests, it is considered with high probability to be prime. Feel free to verify them yourselves on [Wolfram Alpha](https://www.wolframalpha.com/) by simply typing in the prime number.
