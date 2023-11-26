//! Create a random Semaphore Identity
//!
//! Identity secret = Trapdoor + Nullfier
//!
//! Commitment is to be used anywhere you want in public
//! like adding into a MT.

use semaphore::identity::Identity;

fn main() {
    // TODO: Create a function `Identity::random()` with random secret as PR.
    let mut secret = *b"";
    let id = Identity::from_secret(&mut secret, None);
    println!("Id secret: {}", id.secret_hash());
    println!("Trapdoor (secret): {}", id.trapdoor);
    println!("Nullifier (secret): {}", id.nullifier);
    println!("Commitment (public): {}", id.commitment());
}
