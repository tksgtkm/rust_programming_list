fn main() {
    do_it(add, 10, 2);
    do_it(mul, 10, 2);
}

fn do_it(f: fn(u32, u32) -> u32, a : u32, b : u32) {
    println!("{}", f(a, b));
}

fn add(a : u32, b : u32) -> u32 {
    a + b
}

fn mul(a : u32, b : u32) -> u32 {
    a * b
}