# structenv
deserialize environment variables into rust structs and serialize struct to `.env` file.

## note
> All enviroment keys are UPPERCASE

## Usage

```Cargo.toml
[dependencies]
structenv          = "1.0.0"
```

## Example

```rust
use structenv::StructEnv;

#[derive(StructEnv, Debug)]
struct RemoteConfig {
    host: String,
    port: String,
}

#[derive(StructEnv, Debug)]
struct EnvConfig {
    host: String,
    port: i32,
    // env attribute can rename env key
    #[env("SYSTEM_ENABLED")]
    enabled: bool,
    // prefix attribute can set struct field's all fields prefix
    #[prefix("REMOTE_")]
    remote: RemoteConfig,
}

fn main() -> std::io::Result<()> {
    // load env from .env file and enviroment
    let mut config = EnvConfig::load_env()?;
    println!("env:{:?}", &config);
    config.host = "sss.io";
    config.port = 1024;
    // save to .env file
    config.save_env()?;
}
```

```console
$ HOST=x.cc SYSTEM_ENABLED=true REMOTE_HOST=r.cc REMOTE_PORT=66 ./example
env: {
        host: "x.cc", 
        port: 0, 
        enabled: true, 
        remote: {
            host: "r.cc", 
            port: 66
        }
    }
```
after excuted `.env`:
```conf
HOST=sss.io
PORT=1024
SYSTEM_ENABLED=true
REMOTE_HOST=r.cc
REMOTE_HOST=66
```
## License

Licensed under MIT

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
