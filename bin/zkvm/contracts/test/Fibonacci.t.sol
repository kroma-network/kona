// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Fibonacci} from "../src/Fibonacci.sol";
import {SP1VerifierGateway} from "@sp1-contracts/SP1VerifierGateway.sol";

struct SP1ProofFixtureJson {
    bytes32 l1EndBlockHash;
    bytes32 outputRoot;
    bytes32 parentOutputRoot;
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}

contract FibonacciTest is Test {
    using stdJson for string;

    address verifier;
    Fibonacci public fibonacci;

    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/fixtures/fixture.json");
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    function setUp() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        verifier = address(new SP1VerifierGateway(address(1)));
        fibonacci = new Fibonacci(verifier, fixture.vkey);
    }

    function test_ValidFibonacciProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        vm.mockCall(verifier, abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector), abi.encode(true));

        (bytes32 parentOutputRoot, bytes32 outputRoot, bytes32 l1EndBlockHash) = fibonacci.verifyFibonacciProof(fixture.proof, fixture.publicValues);
        assert(parentOutputRoot == fixture.parentOutputRoot);
        assert(outputRoot == fixture.outputRoot);
        assert(l1EndBlockHash == fixture.l1EndBlockHash);
    }

    function testFail_InvalidFibonacciProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        fibonacci.verifyFibonacciProof(fakeProof, fixture.publicValues);
    }
}