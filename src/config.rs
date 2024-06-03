use arith::{VectorizedField, VectorizedM31};

#[derive(Debug, Clone, PartialEq)]
pub enum PolynomialCommitmentType {
    Raw,
    KZG,
    Orion,
    FRI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    M31,
    BabyBear,
    BN254,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FiatShamirHashType {
    SHA256,
    Keccak256,
    Poseidon,
    Animoe,
    MIMC7,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub num_repetitions: usize,
    pub vectorize_size: usize,

    pub field_size: usize,
    pub security_bits: usize,
    pub grinding_bits: usize,
    pub num_parallel: usize, // nb_parallel

    pub polynomial_commitment_type: PolynomialCommitmentType,
    pub field_type: FieldType, // LATER: consider infer this from trait
    pub fs_hash: FiatShamirHashType,
}

impl Default for Config {
    fn default() -> Self {
        Self::m31_config()
    }
}

impl Config {
    pub fn m31_config() -> Self {
        let mut vectorize_size = 0;

        let security_bits = 100;
        let grinding_bits = 10;
        let num_parallel = 16;

        let field_size = match FieldType::M31 {
            FieldType::M31 => {
                vectorize_size = num_parallel / VectorizedM31::PACK_SIZE;
                31
            }
            FieldType::BabyBear => 31,
            FieldType::BN254 => 254,
        };

        let num_repetitions = (security_bits - grinding_bits + field_size - 1) / field_size;

        let polynomial_commitment_type = PolynomialCommitmentType::Raw;
        let field_type = FieldType::M31;
        let fs_hash = FiatShamirHashType::SHA256;

        if polynomial_commitment_type == PolynomialCommitmentType::KZG {
            assert_eq!(field_type, FieldType::BN254);
        }

        Config {
            num_repetitions, // update later
            vectorize_size,  // update later
            field_size,      // update later
            security_bits,
            grinding_bits,
            num_parallel,
            polynomial_commitment_type,
            field_type,
            fs_hash,
        }
    }

    pub fn bn254_config() -> Self {
        let vectorize_size = 1;

        let security_bits = 128;
        let grinding_bits = 0;
        let num_parallel = 16;

        let field_size = 254;

        let num_repetitions = 1;

        let polynomial_commitment_type = PolynomialCommitmentType::Raw;
        let field_type = FieldType::BN254;
        let fs_hash = FiatShamirHashType::SHA256;

        if polynomial_commitment_type == PolynomialCommitmentType::KZG {
            assert_eq!(field_type, FieldType::BN254);
        }

        Config {
            num_repetitions, // update later
            vectorize_size,  // update later
            field_size,      // update later
            security_bits,
            grinding_bits,
            num_parallel,
            polynomial_commitment_type,
            field_type,
            fs_hash,
        }
    }

    #[inline(always)]
    /// return the number of repetitions we will need to achieve security
    pub fn get_num_repetitions(&self) -> usize {
        self.num_repetitions
    }
}
