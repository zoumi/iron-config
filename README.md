#iron_config
This is a config file reader for Iron framework(an web framework writen in Rust).

You can put all you settings in a file which names Iron.toml.

##It will search Iron.toml by this order:

1, `The value of IRON_CONFIG_FILE in environment variable.`

2, `./Iron.toml`

3, `./site/Iron.toml`

##Usage:
In you Cargo.toml:

```toml
    [dependencies]
    iron_config = "0.1.0"
    lazy_static = "*"
```
In you crate:

```rust
    extern crate iron_config;
    use iron_config::IC;
    #[macro_use]
    extern crate lazy_static;
    lazy_static!{
       static ref DOMAIN: &'static str = IC.lookup("MAIN.DOMAIN").unwrap().as_str().unwrap();
     }

    fn main() {
        println!("{:?}",*DOMAIN); 
    }
```
## License

Rocket is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

