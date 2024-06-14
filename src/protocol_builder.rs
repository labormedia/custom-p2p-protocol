use crate::traits::Builder;

///```
/// use p2p_handshake::{
///     traits::Builder,
///     protocol_builder::PayloadBuilder,
/// };
///
/// const SIZE_KNOWN: usize = 24;
/// type SomeType = [u8; SIZE_KNOWN];
/// type SomethingBuild = PayloadBuilder<SomeType>;
///
/// fn build_something() -> SomeType {
///     SomethingBuild::init().build()
/// }
///```
#[derive(Default, Debug)]
pub struct PayloadBuilder<T> {
    pub payload_template: T,
}

impl<T: Default + Clone> Builder for PayloadBuilder<T> {
    type Item = T;
    fn init() -> Self {
        PayloadBuilder {
            payload_template: T::default(),
        }
    }
    fn build(self) -> T {
        self.payload_template.clone()
    }
}