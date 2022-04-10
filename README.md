# INIp
:pencil: An INI parser library written in Rust.

## Summary
   1. [Features](#features)
   2. [Examples](#examples)
   3. [Contributions](#contributions)
   4. [License](#license)

## Features

| Feature                    | Implemented?       |
|----------------------------|--------------------|
| Sections support           | :heavy_check_mark: |
| Disabled entry recognition | :heavy_check_mark: |
| Section nesting support    | :x:                |

## Examples

```ini
; file.ini
[section]
full_name = "Hicaro"
```

```rust
let parsed_file = Parser::parse("file.ini").unwrap();
assert_eq!(parsed_file["section"]["full_name"], "Hicaro".to_string());
```

You can read valid and invalid examples on [`examples`](examples).

## Rules

1. Comment lines start with `;`
   ```ini
   ; this is a comment
   # this is not a comment
   ```

2. All values must be surrounded by quotes

   Valid:
   ```ini
   [section]
   name = "John Doe"
   ```

   Invalid:
   ```ini
   [section]
   name = John Doe
   ```

## Contributions
If you find any problems with this library, please let me know by opening an issue explaining the problem.

## License
This project is licensed under the [MIT](LICENSE) license.
