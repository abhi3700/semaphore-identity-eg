//! Recover Identity from (trapdoor, nullifier)
//!
//! Details: Recover the original Id from trapdoor, nullifier....stored into `semacaulk-blyss` offchain
//! or recovered from modifier version of SSS scheme with minimal cons.

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
    dbg!(id.clone());
    println!("Id secret: {}", id.secret_hash());
    println!("Trapdoor (secret): {}", id.trapdoor);
    println!("Nullifier (secret): {}", id.nullifier);
    println!("Commitment (public): {}", id.commitment());

    // recover the original Id from trapdoor, nullifier
    let recovered_id = Identity {
        trapdoor: id.trapdoor,
        nullifier: id.nullifier,
    };
    // recovered matches with original
    assert_eq!(recovered_id.commitment(), id.commitment());
}
