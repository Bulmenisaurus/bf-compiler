use bf_compiler;

fn main() {
    let bfc = include_str!("./test.bc");
    let compiled_bfc = bf_compiler::compile_bfc(bfc);

    println!("{:?}", compiled_bfc);
}
