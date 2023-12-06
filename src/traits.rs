pub trait EndianWrite {
    type Array;
    fn to_le_bytes(&self) -> Self::Array;
    fn to_be_bytes(&self) -> Self::Array;
}

pub trait EndianRead {
    type Array;
    fn from_le_bytes(array: Self::Array) -> Self;
    fn from_be_bytes(array: Self::Array) -> Self;
}