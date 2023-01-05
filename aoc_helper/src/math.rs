pub fn positive_mod(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}
