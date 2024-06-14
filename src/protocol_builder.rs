use crate::traits::Builder;

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