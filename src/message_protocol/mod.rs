pub trait Message<T> {
    fn serialize(input: T) -> [u8];
    fn deserialize(bytes: [u8]) -> Result<T, String>;
}

enum Operation {
    Get,
    Set,
    Update,
}
pub enum Value {
    Int(i64),
    Text(String),
    Bytes(Vec<u8>),
}
pub enum Request {
    Get(GetRequest),
    Set(SetRequest),
}
pub struct GetRequest {
    pub key: String,
}

pub struct SetRequest {
    pub key: String,
    pub value: Value,
}
pub struct Response {}

// fn foo() {
//     let test = Request { operation: 1 };
//     test.as_bytes();
// }
