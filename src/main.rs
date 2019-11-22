mod bar;
mod foo;

fn main() {
    println!(
        "{}, {}, {} | {}", //
        foo::ONE,
        foo::TWO,
        foo::THREE,
        bar::BAZ
    )
}
