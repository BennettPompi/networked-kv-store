pub trait Message<T> {
    fn serialize(input: T) -> [u8];
    fn deserialize(bytes: [u8]) -> Result<T, String>;
}

enum Operation {
    Get,
    Set,
    Update,
}
pub struct Request {
    operation: Operation,
}
pub struct Response {}

// fn foo() {
//     let test = Request { operation: 1 };
//     test.as_bytes();
// }
