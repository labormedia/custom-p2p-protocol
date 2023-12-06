pub trait EndianWrite {
    type Array;
    fn to_le_bytes(&self) -> Self::Array;
    fn to_be_bytes(&self) -> Self::Array;
}