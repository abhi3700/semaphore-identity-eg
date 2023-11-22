//! Offchain group
//! Create a group with custom depth as 30.

import { Group } from "@semaphore-protocol/group";

// run this, if this file is called directly from command line.
if (require.main === module) {
  // create a group with id 324 and depth 30.
  const group = new Group(324, 30);
  console.log(`Group with id: 324 & depth: 30, : `, group);
}
