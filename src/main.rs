use std::ffi::c_double;

extern "C" {
    fn lib_average(numbers: *const c_double, len: usize) -> c_double;
}

fn rust_average(numbers: &[f64]) -> f64 {
    unsafe { lib_average(numbers.as_ptr(), numbers.len()) }
}

fn main() {
    let numbers = [5.0, 1.0, 8.0, 2.0, 7.0];
    println!("{}", rust_average(&numbers));
}
