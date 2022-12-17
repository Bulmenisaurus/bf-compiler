pub use bf_compiler;

fn main() {
    let code = include_str!("./primality_test.txt");
    let bf = bf_compiler::weird_assembly_to_bf(code);

    println!("{}", bf);
    bf_compiler::execute_bf(bf.as_str());
}
