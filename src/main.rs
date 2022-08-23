use django_code_generator::run_generator;

fn main() {
    let res = run_generator();
    println!("{}", res);
}
