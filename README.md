# osinfo

## Overview

Rust library project used to get operating system detail.

### Library (`osinfo`)

#### `osinfo` usage

To use this crate, add `osinfo` as a dependency to your project's Cargo.toml:

```toml
[dependencies]
osinfo = "1"
```

#### Example

```rust
use osinfo;

let info = osinfo::get();

// Print full information:
println!("OS information: {info}");

println!("ID: {}", info.get_id());
println!("Name: {}", info.get_name());
println!("Version: {}", info.get_version());
println!("Variant: {}", info.get_variant());
println!("Edition: {}", info.get_edition());
println!("Codename: {}", info.get_codename());
```


## License

`osinfo` is licensed under the MIT license. See [LICENSE]([text](https://github.com/codex-jignesh/osinfo/blob/main/LICENSE)) for the details.
