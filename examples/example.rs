use fastcpy::slice_copy;

fn main() {
    let src = [1, 2, 3];
    let mut dst = [0, 0, 3];
    let slice = [2];

    slice_copy(&src[..slice[0]], &mut dst[..slice[0]]);
    dbg!(dst);
}
