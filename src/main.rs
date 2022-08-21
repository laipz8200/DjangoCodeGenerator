use django_code_generator::run_generator;

fn main() {
    let res = run_generator();
    match res {
        Ok(code) => println!("{}", code),
        Err(err) => println!("failed with {}", err),
    }
}
