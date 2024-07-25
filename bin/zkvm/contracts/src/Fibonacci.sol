// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

/// @title Kroma Fault Proof.
/// @author Koma team
/// @notice This contract implements the verification of Kroma’s Fault Proof.
contract Fibonacci {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    address public verifier;

    /// @notice The verification key for the Kroma's Fault Proof program.
    bytes32 public fibonacciProgramVkey;

    constructor(address _verifier, bytes32 _fibonacciProgramVkey) {
        verifier = _verifier;
        fibonacciProgramVkey = _fibonacciProgramVkey;
    }

    /// @notice The entrypoint for verifying the proof of a fibonacci number.
    /// @param proof The encoded proof.
    /// @param publicValues The encoded public values.
    function verifyFibonacciProof(bytes calldata proof, bytes calldata publicValues)
        public
        view
        returns (bytes32, bytes32, bytes32)
    {
        ISP1Verifier(verifier).verifyProof(fibonacciProgramVkey, publicValues, proof);
        (bytes32 parentOutputRoot, bytes32 outputRoot, bytes32 l1EndBlockHash) = abi.decode(publicValues, (bytes32, bytes32, bytes32));
        return (parentOutputRoot, outputRoot, l1EndBlockHash);
    }
}
