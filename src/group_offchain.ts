//! Offchain group
//! By default it uses the depth as 20.

import { Group } from "@semaphore-protocol/group";

// create a group with id 324 and depth 20.
const group = new Group(324);
console.log(`Group: `, group);
