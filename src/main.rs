pub mod ui;

struct A<'testing123_test, '_a, 'hi> {
    a: &'testing123_test str,
    b: &'_a str,
    c: &'hi str,
    d: &'hi str,
}

fn main() {
    let a = '\t';
    //ui::compile();
    ui::meta::generate_meta();
}
