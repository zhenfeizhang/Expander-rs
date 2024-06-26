/// Serde for Fields and Polynomials
pub trait Serde {
    /// serialize self into bytes
    fn serialize_into(&self, buffer: &mut [u8]);

    /// deserialize bytes into field
    fn deserialize_from(buffer: &[u8]) -> Self;
}
