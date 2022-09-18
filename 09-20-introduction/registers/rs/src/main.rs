use std::arch::asm;

fn main() {
    max_u64();
    overflow();
}

fn max_u64() {
    let mut x: u64 = 1;
    unsafe {
        asm!(
            "shl {x}, 63",
            x = inout(reg) x,
        );
    }
    // assert_eq!(x, 18446744073709551615);// u64::MAX
    assert_eq!(x, 9223372036854775808);
}

fn overflow() {
    let mut x: u64 = 1;
    unsafe {
        asm!(
            "shl {x}, 64",
            x = inout(reg) x,
        );
    }
    assert_eq!(x, 1);
}
