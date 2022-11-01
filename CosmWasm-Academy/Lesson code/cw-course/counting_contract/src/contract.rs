use crate::msg::ValueResp;

pub mod query {
    use crate::msg::ValueResp;

    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }

    pub fn Incremented(num: u64) -> ValueResp {
        ValueResp { value: num + 1 }
    }
}