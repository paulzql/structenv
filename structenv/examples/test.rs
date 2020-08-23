use structenv::StructEnv;
use structenv::EnvParser;

#[derive(StructEnv)]
struct TestInner {
    // #[env("LEVEL1")]
    foo: String
}

#[derive(StructEnv)]
struct Test {
    // #[env("LEVEL1")]
    level: i32,
    #[prefix("INNER_")]
    inner: TestInner,
}

fn main() {
    //println!("dir: {}", std::env::current_dir().unwrap().to_str().unwrap());
    //std::env::set_var("INNER_FOO", "WOLRD");
    //println!("LEVEL={}", std::env::var("LEVEL").unwrap());
    let test = Test::load_env().unwrap();
    println!("{}", test.to_env("", ""));
    test.save_env().unwrap();
}