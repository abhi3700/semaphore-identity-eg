import { Identity } from "@semaphore-protocol/identity";

// Represents an identity object.
const { trapdoor, nullifier, commitment } = new Identity();

// print all these values using console.log
console.log(`trapdoor: `, trapdoor);
console.log(`nullifier: `, nullifier);
console.log(`commitment: `, commitment);
