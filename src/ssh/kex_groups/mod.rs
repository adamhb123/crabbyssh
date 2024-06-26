use hex_literal::hex;
use num_bigint::{BigUint, RandBigInt};
use rand;

/* 
    The Diffie-Hellman key exchange provides a shared secret that can not be
    determined by either party alone. The key exchange is combined with a
    signature with the host key to provide host authentication.
*/

// https://datatracker.ietf.org/doc/html/draft-ietf-secsh-transport-09.txt#section-6
pub struct DiffieHellmanGroup {
    pub p: &'static [u8], // Prime (each group defines a shared prime)
    pub g: usize,         // Generator (each group defines a shared generator)
    pub exp_size: u64,
}
pub const DH_GROUP_1: DiffieHellmanGroup = DiffieHellmanGroup {
    p: hex!(
        "
        FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1
         29024E08 8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD
         EF9519B3 CD3A431B 302B0A6D F25F1437 4FE1356D 6D51C245
         E485B576 625E7EC6 F44C42E9 A637ED6B 0BFF5CB6 F406B7ED
         EE386BFB 5A899FA5 AE9F2411 7C4B1FE6 49286651 ECE65381
         FFFFFFFF FFFFFFFF
        "
    )
    .as_slice(),
    g: 2,
    exp_size: 256,
};
pub const DH_GROUP_14: DiffieHellmanGroup = DiffieHellmanGroup {
    p: hex!(
        "
        FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1
        29024E08 8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD
        EF9519B3 CD3A431B 302B0A6D F25F1437 4FE1356D 6D51C245
        E485B576 625E7EC6 F44C42E9 A637ED6B 0BFF5CB6 F406B7ED
        EE386BFB 5A899FA5 AE9F2411 7C4B1FE6 49286651 ECE45B3D
        C2007CB8 A163BF05 98DA4836 1C55D39A 69163FA8 FD24CF5F
        83655D23 DCA3AD96 1C62F356 208552BB 9ED52907 7096966D
        670C354E 4ABC9804 F1746C08 CA18217C 32905E46 2E36CE3B
        E39E772C 180E8603 9B2783A2 EC07A28F B5C55DF0 6F4C52C9
        DE2BCBF6 95581718 3995497C EA956AE5 15D22618 98FA0510
        15728E5A 8AACAA68 FFFFFFFF FFFFFFFF
        "
    )
    .as_slice(),
    g: 2,
    exp_size: 256,
};

pub const DH_GROUP_16: DiffieHellmanGroup = DiffieHellmanGroup {
    p: hex!(
        "
        FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1
        29024E08 8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD
        EF9519B3 CD3A431B 302B0A6D F25F1437 4FE1356D 6D51C245
        E485B576 625E7EC6 F44C42E9 A637ED6B 0BFF5CB6 F406B7ED
        EE386BFB 5A899FA5 AE9F2411 7C4B1FE6 49286651 ECE45B3D
        C2007CB8 A163BF05 98DA4836 1C55D39A 69163FA8 FD24CF5F
        83655D23 DCA3AD96 1C62F356 208552BB 9ED52907 7096966D
        670C354E 4ABC9804 F1746C08 CA18217C 32905E46 2E36CE3B
        E39E772C 180E8603 9B2783A2 EC07A28F B5C55DF0 6F4C52C9
        DE2BCBF6 95581718 3995497C EA956AE5 15D22618 98FA0510
        15728E5A 8AAAC42D AD33170D 04507A33 A85521AB DF1CBA64
        ECFB8504 58DBEF0A 8AEA7157 5D060C7D B3970F85 A6E1E4C7
        ABF5AE8C DB0933D7 1E8C94E0 4A25619D CEE3D226 1AD2EE6B
        F12FFA06 D98A0864 D8760273 3EC86A64 521F2B18 177B200C
        BBE11757 7A615D6C 770988C0 BAD946E2 08E24FA0 74E5AB31
        43DB5BFC E0FD108E 4B82D120 A9210801 1A723C12 A787E6D7
        88719A10 BDBA5B26 99C32718 6AF4E23C 1A946834 B6150BDA
        2583E9CA 2AD44CE8 DBBBC2DB 04DE8EF9 2E8EFC14 1FBECAA6
        287C5947 4E6BC05D 99B2964F A090C3A2 233BA186 515BE7ED
        1F612970 CEE2D7AF B81BDD76 2170481C D0069127 D5B05AA9
        93B4EA98 8D8FDDC1 86FFB7DC 90A6C08F 4DF435C9 34063199
        FFFFFFFF FFFFFFFF
        "
    )
    .as_slice(),
    g: 2,
    exp_size: 512,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiffieHellman {
    p: BigUint,
    g: usize,
    exp_size: u64,
    private_key: BigUint,
    public_key: BigUint,
    shared_secret: BigUint,
}

impl DiffieHellman {
    pub fn new(group: &DiffieHellmanGroup) -> DiffieHellman {
        Self {
            p: BigUint::from_bytes_be(group.p),
            g: group.g,
            exp_size: group.exp_size,
            private_key: BigUint::default(),
            public_key: BigUint::default(),
            shared_secret: BigUint::default(),
        }
    }
    pub fn gen_private_key(&mut self, is_server: bool) -> BigUint {
        let q = (&self.p - &BigUint::from(1u8)) / &BigUint::from(2u8);
        let mut rng = rand::thread_rng();
        self.private_key =
            rng.gen_biguint_range(&if is_server { 1u8.into() } else { 2u8.into() }, &q);
        self.private_key.clone()
    }
    pub fn gen_public_key(&mut self) -> BigUint {
        self.public_key = BigUint::from(self.g).modpow(&self.private_key, &self.p);
        self.public_key.clone()
    }
    pub fn compute_shared_secret(&mut self, other_public_key: BigUint) -> BigUint {
        // = (other_public_key ^ self.private_key) % self.p
        self.shared_secret = other_public_key.modpow(&self.private_key, &self.p);
        self.shared_secret.clone()
    }
    pub fn validate_shared_secret(&self, shared_secret: &BigUint) -> bool {
        // shared_secret must be BOTH:
        //      A. greater than 1
        //      B. less than self.p - 1 (i.e., dh group's shared prime - 1)
        let one = BigUint::from(1u8);
        shared_secret > &one && shared_secret < &(&self.p - &one)
    }
    pub fn decode_public_key(buffer: &[u8]) -> BigUint {
        // Converts public key slice to BigUint
        BigUint::from_bytes_be(buffer)
    }
    pub fn validate_public_key(&self, public_key: &BigUint) -> bool {
        // Public key MUST be BOTH:
        //      A. Greater than 1
        //      B. Less than p (group prime) - 1
        let one = BigUint::from(1u8);
        public_key > &one && public_key < &(&self.p - &one)
    }
}
