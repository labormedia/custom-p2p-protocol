pub trait EndianWrite {
    type Output;
    fn to_le_bytes(&self) -> Self::Output;
    fn to_be_bytes(&self) -> Self::Output;
}

pub trait EndianRead {
    type Input;
    fn from_le_bytes(Output: Self::Input) -> Self;
    fn from_be_bytes(Output: Self::Input) -> Self;
}