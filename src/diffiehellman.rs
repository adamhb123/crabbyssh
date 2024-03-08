use rand::Rng;
use rug::Integer;
use rug::rand::RandState;
/* Prime should be >= 2048 bits (32 bytes)
 * 
 */
fn check_prime<T: Into<u64>>(num: T) -> bool {
    let num = num.into() as u64;
    if num > 1 {
        for i in 2..=((num/2) as u64) {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }
    false
}

fn random_prime() -> u64 {
    /* Prime must be >= 2048 bits (32 bytes)
     * We'll use 4096 bits (64 bytes)
     */
    let mut rng = RandState::new();
    let proposed = rng.bits(4096);
    loop {
        if let IsPrime::Yes = proposed.is_probably_prime(30) {
            proposed
        }
    }

}
fn test_random_prime() {
    println!("Random prime: {}", random_prime())
}


// fn generate_prime(a: ) -> u64 {

// }

// fn primitive_check(g: u64, p: u64) {
//     let arr: Vec<u64> = vec![];
//     for i in 1..p {
//         arr.append(u64::pow(g,i) % p);
//     }
// }

// pub fn get_keys(pubkey_a: u64, pubkey_b: u64) -> (u64, u64) {
//     let p = 
// }