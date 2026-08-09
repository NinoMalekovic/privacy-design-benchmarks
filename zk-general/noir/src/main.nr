struct PrivateToken {
    owner: Field,
    amount: u64,
    commitment: Field,
}

struct Transfer {
    sender: PrivateToken,
    receiver: PrivateToken,
    amount: u64,
    nullifier: Field,
}

// Toy commitment function.
//
// This is NOT cryptographically secure.
// It is only used to demonstrate the structure of a
// private-transfer circuit without external dependencies.
fn compute_commitment(owner: Field, amount: u64, random: Field) -> Field {
    owner * 1234567 + (amount as Field) * 7654321 + random
}

// Toy nullifier function.
//
// A real system should use a cryptographic hash such as
// Poseidon, Pedersen, or another ZK-friendly hash.
fn compute_nullifier(owner: Field, amount: u64, secret: Field) -> Field {
    owner * 1000003 + (amount as Field) * 1000033 + secret
}

fn main(
    // Private witness
    sender_private: Field,
    receiver_private: Field,
    amount: u64,
    random: Field,
    secret: Field,
    // Public inputs
    sender_commitment: pub Field,
    receiver_commitment: pub Field,
    nullifier: pub Field
) {
    // Verify sender commitment.
    let computed_sender_commit = compute_commitment(sender_private, amount, random);

    // Verify receiver commitment.
    let computed_receiver_commit = compute_commitment(receiver_private, amount, random + 1);

    assert(computed_sender_commit == sender_commitment);
    assert(computed_receiver_commit == receiver_commitment);

    // Interpret the private sender value as a balance.
    let sender_balance = sender_private as u64;
    let receiver_balance = receiver_private as u64;

    // Sender must have sufficient funds.
    assert(sender_balance >= amount);

    // Private state transition.
    let new_sender_balance = sender_balance - amount;
    let new_receiver_balance = receiver_balance + amount;

    // Sanity checks.
    assert(new_sender_balance <= sender_balance);
    assert(new_receiver_balance >= receiver_balance);

    // Verify nullifier.
    let computed_nullifier = compute_nullifier(sender_private, amount, secret);

    assert(computed_nullifier == nullifier);
}
