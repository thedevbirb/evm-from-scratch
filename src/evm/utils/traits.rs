use primitive_types::U256;

pub trait Bool {
    fn from_bool(val: bool) -> Self;
}

impl Bool for U256 {
    fn from_bool(val: bool) -> Self {
        if val {
            U256::one()
        } else {
            U256::zero()
        }
    }
}
