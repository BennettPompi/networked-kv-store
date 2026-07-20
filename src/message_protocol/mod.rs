use std::str::FromStr;

const MESSAGE_LENGTH_BYTES: u8 = 8;
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
pub enum TransportLayerError {
    SerializationError(String),
    DeserializationError(String),
}

fn parse_get_request<'a>(
    mut iter: impl Iterator<Item = &'a str>,
) -> Result<GetRequest, ParseRequestError> {
    return match iter.next() {
        Some(key) => Ok(GetRequest {
            key: String::from(key),
        }),
        None => Err(ParseRequestError::MissingArg("No key supplied".to_string())),
    };
}
fn parse_set_request<'a>(
    mut iter: impl Iterator<Item = &'a str>,
) -> Result<SetRequest, ParseRequestError> {
    let key = iter
        .next()
        .ok_or(ParseRequestError::MissingArg("No key supplied".to_string()))?
        .to_string();
    let value = iter
        .next()
        .ok_or(ParseRequestError::MissingArg(
            "No value supplied".to_string(),
        ))?
        .to_string();
    Ok(SetRequest {
        key,
        value: Value::VarChar(value),
    })
}

impl FromStr for Request {
    type Err = ParseRequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, " ");
        let command = parts.next();
        match command.map(|s| s.to_lowercase()).as_deref() {
            Some("set") => Request::Set(parse_set_request(parts)?),
            Some("get") => Request::Get(parse_get_request(parts)?),
            Some(_) => return Err(ParseRequestError::InvalidArg),
            None => return Err(ParseRequestError::MissingArg(String::from("OpType"))),
        };

        Err(ParseRequestError::UnknownType(String::from(
            "Not Implemented",
        )))
    }
}
pub trait Serializable<T, E> {
    fn serialize(input: T) -> Result<Vec<u8>, E>;
    fn deserialize(bytes: &[u8]) -> Result<T, E>;
}
fn serialize_op_code(req: &Request) -> u8 {
    match req {
        Request::Get(_) => 1,
        Request::Set(_) => 2,
    }
}
fn serialize_key_len(req: &Request) -> Result<u8, Box<dyn std::error::Error>> {
    let length = match req {
        Request::Get(req) => req.key.as_str(),
        Request::Set(req) => req.key.as_str(),
    }
    .len()
    .try_into()?;
    Ok(length)
}
fn serialize_key(req: &Request) -> Vec<u8> {
    match req {
        Request::Get(r) => &r.key,
        Request::Set(r) => &r.key,
    }
    .as_bytes()
    .to_vec()
}
fn serialize_value_type(req: &SetRequest) -> u8 {
    match req.value {
        Value::UInt(_) => 1,
        Value::Int(_) => 2,
        Value::Float(_) => 3,
        Value::VarChar(_) => 4,
        Value::Bytes(_) => 5,
    }
}
fn serialize_value(req: &SetRequest) -> Result<Vec<u8>, String> {
    let bytes = match &req.value {
        Value::VarChar(v) => v.as_bytes().to_vec(),
        Value::UInt(v) => v.to_be_bytes().to_vec(),
        Value::Int(v) => v.to_be_bytes().to_vec(),
        Value::Float(v) => v.to_be_bytes().to_vec(),
        Value::Bytes(v) => v.clone(),
    };
    if bytes.len() > u32::MAX as usize {
        return Err("Value exceeds maximum allowed length".to_string());
    }
    Ok(bytes)
}

impl Serializable<Request, String> for Request {
    fn serialize(input: Request) -> Result<Vec<u8>, String> {
        let op_code: u8 = serialize_op_code(&input);
        let (key_length, key): (u8, Vec<u8>) = (
            serialize_key_len(&input).map_err(|_| "Key too long".to_string())?,
            serialize_key(&input),
        );
        let (value_type, value) = match input {
            Request::Set(set) => (
                Some(serialize_value_type(&set)),
                Some(serialize_value(&set)?),
            ),
            Request::Get(_) => (None, None),
        };
        let mut buf: Vec<u8> = Vec::new();
        // placeholder value: we'll fill this in once we've serialized the rest of the message
        buf.extend_from_slice(&[0u8; MESSAGE_LENGTH_BYTES as usize]);

        buf.push(op_code);
        buf.push(key_length);
        buf.extend_from_slice(&key);

        match (value_type, value) {
            (Some(v_type), Some(value)) => {
                buf.push(v_type);
                buf.extend_from_slice(&value);
            }
            (None, None) => {}

            _ => return Err("Invalid value state".to_string()),
        }
        // fill in message length
        let message_length: u64 = (buf.len() - MESSAGE_LENGTH_BYTES as usize) as u64;
        buf[0..8].copy_from_slice(&message_length.to_be_bytes());

        return Ok(buf);
    }

    fn deserialize(bytes: &[u8]) -> Result<Request, String> {
        todo!()
    }
}
