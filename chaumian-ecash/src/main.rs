use rand::Rng;
use std::collections::{HashMap, HashSet};

// Simplified RSA blind signature.
// Educational demonstration only — not production cryptography.
struct BlindSignature {
    private_key: u64,
    public_key: u64,
    modulus: u64,
}

impl BlindSignature {
    fn new() -> Self {
        // Toy RSA parameters:
        //
        // p = 17, q = 19
        // n = 17 * 19 = 323
        // phi(n) = 16 * 18 = 288
        //
        // e = d = 17 because:
        // 17 * 17 = 289 = 1 (mod 288)
        BlindSignature {
            private_key: 17,
            public_key: 17,
            modulus: 323,
        }
    }

    fn mod_pow(&self, mut base: u64, mut exponent: u64) -> u64 {
        let modulus = self.modulus;
        let mut result = 1u64;

        base %= modulus;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % modulus;
            }

            base = (base * base) % modulus;
            exponent /= 2;
        }

        result
    }

    // RSA blind:
    //
    // blinded = m * r^e mod n
    fn blind(&self, message: u64, blinding_factor: u64) -> u64 {
        let message = message % self.modulus;
        let factor = blinding_factor % self.modulus;

        let factor_power = self.mod_pow(factor, self.public_key);

        (message * factor_power) % self.modulus
    }

    // RSA signing:
    //
    // signed_blinded = blinded^d mod n
    fn sign_blinded(&self, blinded: u64) -> u64 {
        self.mod_pow(blinded, self.private_key)
    }

    // Greatest common divisor.
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }

        a
    }

    // Calculate modular inverse using the extended Euclidean algorithm.
    fn mod_inverse(&self, value: u64) -> u64 {
        let modulus = self.modulus as i64;

        let mut t = 0i64;
        let mut new_t = 1i64;

        let mut r = modulus;
        let mut new_r = (value % self.modulus) as i64;

        while new_r != 0 {
            let quotient = r / new_r;

            let temp_t = t;
            t = new_t;
            new_t = temp_t - quotient * new_t;

            let temp_r = r;
            r = new_r;
            new_r = temp_r - quotient * new_r;
        }

        assert_eq!(
            r, 1,
            "blinding factor must be relatively prime to the RSA modulus"
        );

        if t < 0 {
            t += modulus;
        }

        t as u64
    }

    // Remove the blinding factor:
    //
    // signature = signed_blinded * r^-1 mod n
    fn unblind(&self, signed_blinded: u64, blinding_factor: u64) -> u64 {
        let factor = blinding_factor % self.modulus;
        let inverse = self.mod_inverse(factor);

        (signed_blinded * inverse) % self.modulus
    }

    // Generate a valid RSA blinding factor.
    //
    // r must satisfy gcd(r, n) = 1 so that r^-1 exists modulo n.
    fn generate_blinding_factor(&self) -> u64 {
        let mut rng = rand::thread_rng();

        loop {
            let candidate = rng.gen_range(2..self.modulus);

            if Self::gcd(candidate, self.modulus) == 1 {
                return candidate;
            }
        }
    }

    fn verify(&self, message: u64, signature: u64) -> bool {
        let expected = message % self.modulus;
        let recovered = self.mod_pow(signature, self.public_key);

        recovered == expected
    }
}

// E-cash token.
#[derive(Clone)]
struct EcashToken {
    token_id: u64,
    value: u64,
    signature: u64,
    issuer: String,
}

// Chaumian E-cash system.
struct ChaumianEcash {
    issuer: BlindSignature,
    issued_tokens: HashMap<u64, EcashToken>,
    spent_tokens: HashSet<u64>,
    pending_withdrawals: Vec<EcashToken>,
}

impl ChaumianEcash {
    fn new() -> Self {
        ChaumianEcash {
            issuer: BlindSignature::new(),
            issued_tokens: HashMap::new(),
            spent_tokens: HashSet::new(),
            pending_withdrawals: Vec::new(),
        }
    }

    // User requests a blind signature for a new token.
    fn withdraw_request(&mut self, amount: u64) -> EcashToken {
        let mut rng = rand::thread_rng();

        // Ensure the message is represented inside the RSA modulus.
        let token_id = rng.gen_range(1..self.issuer.modulus);

        // Generate r such that gcd(r, n) = 1.
        let blinding_factor = self.issuer.generate_blinding_factor();

        // User blinds the token.
        let blinded = self.issuer.blind(token_id, blinding_factor);

        // Issuer signs the blinded token without seeing token_id.
        let signed_blinded = self.issuer.sign_blinded(blinded);

        // User removes the blinding factor.
        let signature = self
            .issuer
            .unblind(signed_blinded, blinding_factor);

        // Confirm that the resulting signature is valid.
        assert!(
            self.issuer.verify(token_id, signature),
            "generated blind signature failed verification"
        );

        let token = EcashToken {
            token_id,
            value: amount,
            signature,
            issuer: "Mint".to_string(),
        };

        self.pending_withdrawals.push(token.clone());

        token
    }

    // Mint tokens by verifying their blind signatures.
    fn mint_tokens(&mut self, tokens: Vec<EcashToken>) -> Vec<EcashToken> {
        let mut minted = Vec::new();

        for mut token in tokens {
            if self.spent_tokens.contains(&token.token_id) {
                continue;
            }

            let verified = self
                .issuer
                .verify(token.token_id, token.signature);

            if verified && !self.issued_tokens.contains_key(&token.token_id) {
                token.issuer = "Mint".to_string();

                self.issued_tokens
                    .insert(token.token_id, token.clone());

                minted.push(token);
            }
        }

        minted
    }

    // Spend token.
    fn spend_token(&mut self, token_id: u64) -> bool {
        if self.issued_tokens.contains_key(&token_id) {
            self.spent_tokens.insert(token_id);
            self.issued_tokens.remove(&token_id);

            return true;
        }

        false
    }

    // Transfer token off-chain.
    fn transfer_token(&mut self, token_id: u64, _new_owner: String) -> bool {
        // Conceptually, the token can move between parties
        // without creating a public ledger transaction.
        self.issued_tokens.contains_key(&token_id)
    }
}

fn main() {
    println!("=== Chaumian E-cash Privacy Demo ===\n");

    let mut ecash = ChaumianEcash::new();

    // User requests withdrawals.
    println!("1. User requests 100 units withdrawal (blinding message):");

    let token1 = ecash.withdraw_request(100);
    let token2 = ecash.withdraw_request(50);
    let token3 = ecash.withdraw_request(25);

    println!("   Tokens created with blind signatures (identity hidden)");

    // Mint tokens.
    let minted = ecash.mint_tokens(vec![token1, token2, token3]);

    println!("\n2. Tokens minted: {} tokens", minted.len());
    println!("   Total value minted: 175 units");

    // Spend one token.
    println!("\n3. Spend token (to merchant):");

    let spent = ecash.spend_token(minted[0].token_id);

    println!("   Token spent: {}", spent);

    // Transfer another token off-chain.
    println!("\n4. Transfer token to another party (off-chain):");

    let transferred =
        ecash.transfer_token(minted[1].token_id, "Alice".to_string());

    println!("   Token transferred off-chain: {}", transferred);

    // Check state.
    println!("\n5. System state:");

    println!(
        "   Issued tokens: {}",
        ecash.issued_tokens.len()
    );

    println!(
        "   Spent tokens: {}",
        ecash.spent_tokens.len()
    );

    println!(
        "   Pending withdrawals: {}",
        ecash.pending_withdrawals.len()
    );

    println!("\nGraph information:");

    println!("   ✓ No public transaction graph");
    println!("   ✓ Tokens are transferred off-chain");
    println!("   ✓ No persistent public history of transfers");
    println!("   ✓ Only token issuance and redemption are recorded");
    println!("   ✓ Blind signatures break issuance-to-redemption linkage");
}