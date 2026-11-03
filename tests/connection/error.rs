#[derive(Debug)]
pub struct Error {
    pub source: abi::HoleFail,
}

pub(crate) fn denied() -> Error {
    Error {
        source: abi::HoleFail::Denied,
    }
}
