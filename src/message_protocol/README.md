# Key Value Message Protocol Structure

## REQUEST:
  ```
  HEADER:  
  | MESSAGE_LENGTH (u64) | OP_CODE (u8) | KEY_LENGTH (u8) |
  CONTENT: 
  | KEY (bytes[KEY_LENGTH]) ( [ | VALUE_TYPE (u8) | VALUE (bytes[]) | ], optional) 
  ```

## RESPONSE:
```
  HEADER:  | MESSAGE_LENGTH (u64) | STATUS_CODE (u8) | VALUE_TYPE (u8) 
  CONTENT: | VALUE (bytes[]) |
```

## Type Defs:
```
  OP_CODE: enum { GET, SET, UPDATE}
  VALUE_TYPE: enum { STRING, INT, UINT, BYTES, EXPRESSION }
```
## Examples
```
  SET abc 123 -> OK, 123
  GET abc -> OK, 123
  UPDATE abc n' = n * 2 -> OK, 246
```
