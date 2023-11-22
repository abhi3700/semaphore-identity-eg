//! This file is for generating an identity object with a secret message.

import { Identity } from "@semaphore-protocol/identity";

/// @dev This is to generate deterministic identity each time you run the script.
export function getCommitmentFromSecretMessage(secretMessage: string): string {
  const { trapdoor, nullifier, commitment } = new Identity(secretMessage);
  return commitment.toString();
}

// run this script only if this file is called directly from command line.
if (require.main === module) {
  // Represents an identity object with a secret message like seed phrase.
  // the secret message could be some message to be signed or a seed phrase.
  const identity = new Identity(
    "lock frost nation imitate party medal knee cigar rough wine document immense"
  );
  console.log(`Identity: `, identity);

  // Print the trapdoor and nullifier of the identity as
  // poseidon2 hashes of the respective integers.
  console.log(`Identity with trapdoor & nullfier: `, identity.toString());

  // print all these values.
  console.log(`trapdoor (secret): `, identity.trapdoor);
  console.log(`nullifier (secret): `, identity.nullifier);
  console.log(`commitment (public): `, identity.commitment);

  // recover the identity
  const recoveredIdentity = new Identity(identity.toString());
  console.log("Recovered identity from string: ", recoveredIdentity.toString());
}
