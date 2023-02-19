
pub use structenv_derive::StructEnv;
pub use dotenv;
use std::collections::HashMap;

pub trait EnvParser: Sized {
    fn read_env(key: &str, prefix: &str) -> std::io::Result<Self>;
    fn to_env(&self, _key: &str, prefix: &str) -> HashMap<String, String>;
}

impl <T> EnvParser for T
    where T: std::str::FromStr + std::default::Default + std::string::ToString
    {
        fn read_env(key: &str, prefix: &str) -> std::io::Result<Self> {
            parse_env::<Self>(&format!("{}{}", prefix, key))
        }
        fn to_env(&self, key: &str, prefix: &str) -> HashMap<String, String> {
            let mut map = HashMap::new();
            map.insert(format!("{}{}", prefix, key), self.to_string());
            map
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
    let mut envs: std::collections::HashMap<String,String> = parser.to_env("", "");
    for (k, v) in envs.iter() {
        std::env::set_var(k, v);
    }
    let mut lines: Vec<String> = std::fs::read_to_string(filename)?.lines().map(|line| {
        let kv: Vec<&str> = line.splitn(2, "=").collect();
        let key = kv[0].to_string();
        if let Some(value) = envs.remove(&key) {
            format_env(&key, value.as_str())
        } else {
            line.to_string()
        }
    }).collect();
    for (k, v) in envs.iter() {
        lines.push(format_env(&k, &v));
    }
    std::fs::write(filename, lines.join("\n"))    
}

fn format_env(k: &str, v: &str) -> String {
    if v.find(" ").is_some() || v.find("=").is_some() || v.find("\n").is_some() {
        format!("{}={:?}", k, v)
    } else {
        format!("{}={}", k, v)
    }
}

#[test]
fn test_loader() {
    use EnvParser;

    let x: i32 = 100;
    assert_eq!(x.to_env("V", ""), "V=100");

    let y: String = "world".to_string();
    assert_eq!(y.to_env("HELLO", ""), "HELLO=world");
}
