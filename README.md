#iron-config
This is a config file reader for Iron framework(an web framework writen in Rust).

You can put all you setting in a file which names Iron.toml.

#It will search Iron.toml by this order:

1, The value of IRON_CONFIG_FILE in environment variable.

2, ./Iron.toml

3, ./site/Iron.toml

#Usage:
In you Cargo.toml:

```toml
    [dependencies]
    iron-config = "0.1.0"
    lazy_static = "*"
```
In you crate:

```rust
    extern crate iron-config;
    use iron-config::IC;
    lazy_static!{
        static ref DOMAIN: &'static str = IC.lookup("MAIN.DOMAIN").unwrap().as_str().unwrap();
    }
    prinln!("{:?}",*DOMAIN);
```


