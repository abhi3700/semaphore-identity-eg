//! This is to generate random identity each time you run the script.
import { Identity } from "@semaphore-protocol/identity";

// Represents an identity object.
const { trapdoor, nullifier, commitment } = new Identity();

// print all these values using console.log
console.log(`trapdoor (secret): `, trapdoor);
console.log(`nullifier (secret): `, nullifier);
console.log(`commitment (public): `, commitment);
