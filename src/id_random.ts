//! This is to generate random identity each time you run the script.
import { Identity } from "@semaphore-protocol/identity";

/// @dev This is to generate random identity each time you run the script.
export function getCommitment(): string {
  const { trapdoor, nullifier, commitment } = new Identity();
  return commitment.toString();
}

export function getCommitmentBigint(): bigint {
  const { trapdoor, nullifier, commitment } = new Identity();
  return commitment;
}

// run this, if this file is called directly from command line.
if (require.main === module) {
  // Represents an identity object.
  const { trapdoor, nullifier, commitment } = new Identity();

  // print all these values using console.log
  console.log(`trapdoor (secret): `, trapdoor);
  console.log(`nullifier (secret): `, nullifier);
  console.log(`commitment (public): `, commitment);
}
