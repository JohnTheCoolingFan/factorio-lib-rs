pub const fn default_i128<const V: i128>() -> i128 {
    V
}
pub const fn default_i64<const V: i64>() -> i64 {
    V
}
pub const fn default_i32<const V: i32>() -> i32 {
    V
}
pub const fn default_i16<const V: i16>() -> i16 {
    V
}
pub const fn default_i8<const V: i8>() -> i8 {
    V
}
pub const fn default_u128<const V: u128>() -> u128 {
    V
}
pub const fn default_u64<const V: u64>() -> u64 {
    V
}
pub const fn default_u32<const V: u32>() -> u32 {
    V
}
pub const fn default_u16<const V: u16>() -> u16 {
    V
}
pub const fn default_u8<const V: u8>() -> u8 {
    V
}
pub const fn default_bool<const V: bool>() -> bool {
    V
}
pub const fn default_char<const V: char>() -> char {
    V
}
pub fn default_from_i128<T: From<i128>, const V: i128>() -> T {
    V.into()
}
pub fn default_from_i64<T: From<i64>, const V: i64>() -> T {
    V.into()
}
pub fn default_from_i32<T: From<i32>, const V: i32>() -> T {
    V.into()
}
pub fn default_from_i16<T: From<i16>, const V: i16>() -> T {
    V.into()
}
pub fn default_from_i8<T: From<i8>, const V: i8>() -> T {
    V.into()
}
pub fn default_from_u128<T: From<u128>, const V: u128>() -> T {
    V.into()
}
pub fn default_from_u64<T: From<u64>, const V: u64>() -> T {
    V.into()
}
pub fn default_from_u32<T: From<u32>, const V: u32>() -> T {
    V.into()
}
pub fn default_from_u16<T: From<u16>, const V: u16>() -> T {
    V.into()
}
pub fn default_from_u8<T: From<u8>, const V: u8>() -> T {
    V.into()
}
