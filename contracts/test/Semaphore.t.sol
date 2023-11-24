// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {Semaphore} from "../src/Semaphore.sol";

contract SemaphoreTest is Test {
    Semaphore public semaphore;

    function setUp() public {
        semaphore = new Semaphore(/* Add verifier address */);
    }

    function test_create_group() public {}

    function testFuzz_create_group_duration(uint256 x) public {}
}
