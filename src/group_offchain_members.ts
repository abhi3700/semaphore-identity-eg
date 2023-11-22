//! Offchain group
//! Create a group with custom depth as 30 & add members to it.
//! Add, Remove, Update members to this group offchain.

import { Group } from "@semaphore-protocol/group";
import { getCommitment, getCommitmentBigint } from "./id_random";

// main function
if (require.main === module) {
  // add members to an array
  // populate an empty array with given length & elements with same function call called 10 times.
  const members = Array.from({ length: 10 }, () => getCommitment());

  // create a group with id 324 and depth 30, with members.
  console.log(
    "===Group created with id: 324 & depth: 30, initialized with 30 members commitments==="
  );
  const group = new Group(324, 30, members);
  console.log(`Group at the beginning: `, group);
  console.log(`Members: `, group.members);

  // add a new member to this group.
  console.log("===Adding a new member to this group===");
  group.addMember(getCommitment());
  console.log(`Members: `, group.members);

  // remove 1-indexed member from this group.
  // NOTE: Removing a member from a group sets the node value to a special value (i.e. zeroValue)
  // provided the node isn't removed, and the length of the group.members array doesn't change.
  console.log("===Removing 1-indexed member from this group===");
  group.removeMember(1);
  console.log(`Members: `, group.members);

  // remove 4-indexed member from this group.
  console.log("===Removing 4-indexed member from this group===");
  group.removeMember(4);
  console.log(`Members: `, group.members);

  // remove 4-indexed member from this group.
  // NOTE: this won't throw any error even if you try to remove a member which is not present in the group.
  console.log("===Removing 4-indexed member from this group===");
  group.removeMember(4);
  console.log(`Members: `, group.members);

  // update 0-indexed (existing) member with a new value.
  console.log("===Updating 0-indexed (existing) member with a new value===");
  group.updateMember(0, 24556);
  console.log(`Members: `, group.members);

  // update 0-indexed (existing) member with a new commitment.
  console.log(
    "===Updating 0-indexed (existing) member with a new commitment==="
  );
  group.updateMember(0, getCommitmentBigint());
  console.log(`Members: `, group.members);

  // update 1-indexed member (removed) with a new value or commitment.
  console.log(
    "===Updating 1-indexed (removed) member with a new commitment==="
  );
  group.updateMember(1, getCommitmentBigint());
  console.log(`Members: `, group.members);

  console.log(`Group at the end: `, group);
}
