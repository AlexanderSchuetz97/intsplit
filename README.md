# intsplit
Rust library for splitting numeric types into their binary component arrays.

### Example
```rust
use intsplit::*;

#[test]
fn example() {
    let number = 0x0A0B0C0Du32;
    let u16_components: [u16; 2] = number.as_u16_array();
    assert_eq!([0x0C0Du16, 0x0A0Bu16], u16_components);
}
```

### Implemented transmutes
This library uses no unsafe code (only in tests) to implement the transmutes.
The rust compiler will recognize that all these operations are essentially transmutes 
and optimize similarly (on optimized builds)

#### u16/i16:
- as_i8_array -> [i8; 2]

#### u32/i32/f32:
- as_i8_array -> [i8; 4]
- as_i16_array -> [i16; 2]
- as_u16_array -> [u16; 2]

#### u64/i64/f64:
- as_i8_array -> [i8; 8]
- as_i16_array -> [i16; 4]
- as_u16_array -> [u16; 4]
- as_i32_array -> [i32; 2]
- as_u32_array -> [u32; 2]
- as_f32_array -> [f32; 2]


#### u128/i128:
- as_i8_array -> [i8; 16]
- as_i16_array -> [i16; 8]
- as_u16_array -> [u16; 8]
- as_i32_array -> [i32; 4]
- as_u32_array -> [u32; 4]
- as_f32_array -> [f32; 4]
- as_i64_array -> [i64; 2]
- as_u64_array -> [u64; 2]
- as_f64_array -> [f64; 2]

### Beware of endianness
Please note that the result is always equivalent to transmuting the to_ne_bytes() result to the respective array.
The example provided in this readme will not work on big endian targets as that would swap the 2 u16 values in the assert statement,
just like mem::transmute - ing the result of to_ne_bytes() would not yield the same result on little and big endian. 

