pub trait Features {
    fn to_vec(&self) -> Vec<f32>;
    fn to_vec_without(&self, fields: &[&str]) -> Vec<f32>;
    fn names() -> Vec<&'static str>;
}

pub trait ToOneHot {
    type OneHotStruct;
    fn to_one_hot(&self) -> Self::OneHotStruct;
}
