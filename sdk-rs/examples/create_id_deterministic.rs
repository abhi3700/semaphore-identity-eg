//! Create a Semaphore Identity from a given seed phrase.
//! Hence, deterministic identity.
//!
//! Identity secret = Trapdoor + Nullfier
//!
//! Commitment is to be used anywhere you want in public
//! like adding into a MT.

use semaphore::identity::Identity;

fn main() {
    let mut secret =
        *b"lock frost nation imitate party medal knee cigar rough wine document immense";
    let id = Identity::from_secret(&mut secret, None);
    println!("Id secret: {}", id.secret_hash());
    println!("Trapdoor (secret): {}", id.trapdoor);
    println!("Nullifier (secret): {}", id.nullifier);
    println!("Commitment (public): {}", id.commitment());
}
