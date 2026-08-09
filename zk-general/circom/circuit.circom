// ZK circuit for private state transition
// Demonstrates general-purpose private computation

pragma circom 2.1.0;

include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";


// ============================================================
// Private state transition
// ============================================================

template PrivateStateTransition() {

    // Private inputs
    signal input current_state;
    signal input transition[3];
    signal input witness[3];

    // Public inputs
    signal input public_hash;
    signal input public_result;
    signal input public_state_root;

    // Internal signals
    signal computed_hash;
    signal computed_result;
    signal computed_state_root;


    // Hash private state and transition parameters

    component state_hash = Poseidon(4);

    state_hash.inputs[0] <== current_state;
    state_hash.inputs[1] <== transition[0];
    state_hash.inputs[2] <== transition[1];
    state_hash.inputs[3] <== transition[2];

    computed_hash <== state_hash.out;

    computed_hash === public_hash;


    // Compute state transition

    component state_calc = StateCalculator();

    state_calc.current <== current_state;
    state_calc.transition0 <== transition[0];
    state_calc.transition1 <== transition[1];
    state_calc.transition2 <== transition[2];

    computed_result <== state_calc.out;

    computed_result === public_result;


    // Verify state root

    component merkle_proof = MerkleInclusion(3);

    merkle_proof.leaf <== computed_result;
    merkle_proof.siblings[0] <== witness[0];
    merkle_proof.siblings[1] <== witness[1];
    merkle_proof.siblings[2] <== witness[2];

    computed_state_root <== merkle_proof.out;

    computed_state_root === public_state_root;
}


// ============================================================
// State transition component
// ============================================================

template StateCalculator() {

    signal input current;
    signal input transition0;
    signal input transition1;
    signal input transition2;

    signal output out;

    signal intermediate[3];

    intermediate[0] <== current + transition0;
    intermediate[1] <== intermediate[0] * transition1;
    intermediate[2] <== intermediate[1] - transition2;

    out <== intermediate[2];
}


// ============================================================
// Merkle inclusion component
// ============================================================

template MerkleInclusion(levels) {

    assert(levels == 3);

    signal input leaf;
    signal input siblings[levels];

    signal output out;


    // Level 0

    component hash0 = Poseidon(1);

    hash0.inputs[0] <== leaf;


    // Level 1

    component hash1 = Poseidon(2);

    hash1.inputs[0] <== hash0.out;
    hash1.inputs[1] <== siblings[0];


    // Level 2

    component hash2 = Poseidon(2);

    hash2.inputs[0] <== hash1.out;
    hash2.inputs[1] <== siblings[1];


    // Level 3

    component hash3 = Poseidon(2);

    hash3.inputs[0] <== hash2.out;
    hash3.inputs[1] <== siblings[2];


    // Final state root

    out <== hash3.out;
}


// ============================================================
// Main circuit
// ============================================================

component main {
    public [
        public_hash,
        public_result,
        public_state_root
    ]
} = PrivateStateTransition();