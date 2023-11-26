//! E2E testing for [`Semaphore`] Identity

use semaphore::{
    get_supported_depths, hash_to_field, identity::Identity, poseidon_tree::LazyPoseidonTree,
    protocol::*, Field,
};

fn main() {
    // generate identity
    let mut secret = *b"secret";
    let id = Identity::from_secret(&mut secret, None);

    // Get the first available tree depth. This is controlled by the crate features.
    let depth = get_supported_depths()[0];
    dbg!(depth);

    // generate merkle tree with depth as 30, leaves as GF(0).
    // TODO: Currently the Field is based on BN254 curve, but can be improved with BLS12-381 curve.
    let leaf = Field::from(0);
    let mut tree = LazyPoseidonTree::new(depth, leaf).derived();
    // add a member commitment to the tree
    // FIXME: This is problematic for onchain activities as it is disclosing
    // index with corresponding commitment.
    // Solved by [`Semacaulk`].
    tree = tree.update(0, &id.commitment());

    // generate merkle proof for the member (at index 0).
    let merkle_proof = tree.proof(0);
    let root = tree.root();
    dbg!(root);

    // change signal and external_nullifier here
    let signal_hash = hash_to_field(b"abhi");
    // set anything as salt (may be) for hashing with id.
    let external_nullifier_hash = hash_to_field(b"appId1");
    dbg!(external_nullifier_hash);

    let nullifier_hash = generate_nullifier_hash(&id, external_nullifier_hash);

    let proof = generate_proof(&id, &merkle_proof, external_nullifier_hash, signal_hash).unwrap();
    dbg!(proof);
    let success = verify_proof(
        root,
        nullifier_hash,
        signal_hash,
        external_nullifier_hash,
        &proof,
        depth,
    )
    .unwrap();

    assert!(success);
}
