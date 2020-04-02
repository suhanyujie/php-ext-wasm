#[no_mangle]
pub extern "C" fn sum(x: i32, y: i32) -> i32 {
    extra_sum(x, y)
}

fn extra_sum(x: i32, y: i32) -> i32 {
    x + y + 1000
}
