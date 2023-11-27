//! Create a random Semaphore Identity
//!
//! Identity secret = Trapdoor + Nullfier
//!
//! Commitment is to be used anywhere you want in public
//! like adding into a MT.
//!
//! PR submitted: https://github.com/worldcoin/semaphore-rs/pull/59
//!
//! ## Use case
//!
//! - Developing a method to assign unique IDs (commitments)
//! to users and securely manage their identity
//! secrets (trapdoor & nullifier) in a custodial setup.

use rand::Rng;
use semaphore::identity::Identity;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen();

    // num to string to bytes
    let random_string = random_number.to_string();
    let secret_bytes = random_string.as_bytes();

    // convert bytes to a fixed-length byte array, you'll need to adjust this.
    let mut secret = Vec::from(secret_bytes);
    let id = Identity::from_secret(&mut secret, None);
    println!("Id secret: {}", id.secret_hash());
    println!("Trapdoor (secret): {}", id.trapdoor);
    println!("Nullifier (secret): {}", id.nullifier);
    println!("Commitment (public): {}", id.commitment());
}
