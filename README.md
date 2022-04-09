# INIp
:pencil: An INI parser library written in Rust.

## Features

| Feature                    | Implemented?       |
|----------------------------|--------------------|
| Sections support           | :heavy_check_mark: |
| Disabled entry recognition | :heavy_check_mark: |

## Examples

```ini
; file.ini
[section]
full_name = "Hicaro"
```

```rust
let parsed_file = Parser::parse("examples/valid/example4.ini").unwrap();
assert_eq!(parsed_file["section"]["full_name"], "Hicaro".to_string());
```

You can read valid and invalid examples on [`examples`](examples).

## License
This project is licensed under the [MIT](LICENSE) license.
