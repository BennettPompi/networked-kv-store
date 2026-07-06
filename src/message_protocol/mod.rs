use std::str::FromStr;
pub enum Value {
    UInt(u32),
    Int(i32),
    Float(f32),
    VarChar(String),
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
pub enum ParseRequestError {
    InvalidArg,
    MissingArg(String),
    UnknownType(String),
}
fn parse_get_request<'a>(mut iter: impl Iterator<Item = &'a str>) -> GetRequest {
    // TODO: handle this
    let key = String::from(iter.next().unwrap());
    return GetRequest { key: key };
}
fn parse_set_request<'a>(mut iter: impl Iterator<Item = &'a str>) -> SetRequest {
    // TODO: handle this
    let key = String::from(iter.next().unwrap());
    // TODO: handle this
    let value = String::from(iter.next().unwrap());
    // TODO: Add type inference / specification system
    return SetRequest {
        key: key,
        value: Value::VarChar(value),
    };
}

pub trait Serializable<T> {
    fn serialize(input: T) -> [u8];
    fn deserialize(bytes: &[u8]) -> Result<T, String>;
}
impl FromStr for Request {
    type Err = ParseRequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, " ");
        let command = parts.next();
        match command.map(|s| s.to_lowercase()).as_deref() {
            Some("set") => Request::Set(parse_set_request(parts)),
            Some("get") => Request::Get(parse_get_request(parts)),
            Some(_) => return Err(ParseRequestError::InvalidArg),
            None => return Err(ParseRequestError::MissingArg(String::from("OpType"))),
        };

        Err(ParseRequestError::UnknownType(String::from(
            "Not Implemented",
        )))
    }
}
