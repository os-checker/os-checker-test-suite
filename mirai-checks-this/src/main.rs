pub fn to_be(dst: &mut [u16]) {
    for v in dst.iter_mut() {
        *v = v.to_be();
    }
}

pub fn main() {
    to_be(&[]);
}
