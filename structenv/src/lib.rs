
pub use structenv_derive::StructEnv;
pub use dotenv;

pub trait EnvParser: Sized {
    fn read_env(key: &str, prefix: &str) -> std::io::Result<Self>;
    fn to_env(&self, _key: &str, prefix: &str) -> String;
}

impl <T> EnvParser for T
    where T: std::str::FromStr + std::default::Default + std::string::ToString
    {
        fn read_env(key: &str, prefix: &str) -> std::io::Result<Self> {
            parse_env::<Self>(&format!("{}{}", prefix, key))
        }
        fn to_env(&self, key: &str, prefix: &str) -> String {
            format!("{}{}={}", prefix, key, self.to_string())
        } 
    }

pub fn parse_env<T: std::str::FromStr + std::default::Default>(key: &str) -> std::io::Result<T> {
    std::env::var(key)
        .map_or(Ok(std::default::Default::default()), |v| {
            v.parse::<T>().map_err(|_e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Parse env {} failed", key)))
        })
}

pub fn save_env<T: EnvParser>(parser: &T, filename: &str) -> std::io::Result<()> {
    if !std::path::Path::new(filename).exists() {
        std::fs::File::create(filename).ok();
    }
    let mut envs: std::collections::HashMap<String,String> = parser.to_env("", "").lines().map(|line| {
        let kv: Vec<&str> = line.splitn(2, "=").collect();
        std::env::set_var(kv[0], kv[1]);
        (kv[0].to_string(), kv[1].to_string())
    }).collect();
    let mut lines: Vec<String> = std::fs::read_to_string(filename)?.lines().map(|line| {
        let kv: Vec<&str> = line.splitn(2, "=").collect();
        let key = kv[0].to_string();
        if let Some(value) = envs.remove(&key) {
            if value.as_str().find(" ").is_some() || value.as_str().find("=").is_some() {
                return format!("{}={:?}", key, value);
            }
            return format!("{}={}", key, value);
        }
        line.to_string()
    }).collect();
    for (k, v) in envs.iter() {
        lines.push(format!("{}={}", k, v));
    }
    std::fs::write(filename, lines.join("\n"))    
}

#[test]
fn test_loader() {
    use EnvParser;

    let x: i32 = 100;
    assert_eq!(x.to_env("V", ""), "V=100");

    let y: String = "world".to_string();
    assert_eq!(y.to_env("HELLO", ""), "HELLO=world");
}