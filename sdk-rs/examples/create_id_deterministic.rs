//! Create a Semaphore Identity from a given seed phrase.
//! Hence, deterministic identity.
//!
//! Identity secret = Trapdoor + Nullfier
//!
//! Commitment is to be used anywhere you want in public
//! like adding into a MT.

use semaphore::identity::Identity;

fn main() {
    let seed_phrase =
        "lock frost nation imitate party medal knee cigar rough wine document immense";
    println!("Seed phrase: {}", seed_phrase);
    let binding = seed_phrase.to_string();
    let secret_bytes = binding.as_bytes();

    let mut secret = Vec::from(secret_bytes);
    let id = Identity::from_secret(&mut secret, None);
    println!("Id secret: {}", id.secret_hash());
    println!("Trapdoor (secret): {}", id.trapdoor);
    println!("Nullifier (secret): {}", id.nullifier);
    println!("Commitment (public): {}", id.commitment());
}
