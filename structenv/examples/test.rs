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
    str: String,
}

fn main() {
    //println!("dir: {}", std::env::current_dir().unwrap().to_str().unwrap());
    //std::env::set_var("INNER_FOO", "WOLRD");
    //println!("LEVEL={}", std::env::var("LEVEL").unwrap());
    let path = "target/debug/test.env";
    let mut test = Test::load_env(path).unwrap();
    println!("{:?}", test.to_env("", ""));
    test.inner.foo = "hello world".to_string();
    test.str = "hello\n-world".to_string();
    test.save_env(path).unwrap();
}
