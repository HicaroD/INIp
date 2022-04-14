# INIp
:pencil: An INI parser library written in Rust.

## Summary
   1. [Features](#features)
   2. [Installation](#installation)
   3. [Examples](#examples)
   4. [Rules](#rules)
   5. [Contributions](#contributions)
   6. [License](#license)

## Features

| Feature                    | Implemented?       |
|----------------------------|--------------------|
| Sections support           | :heavy_check_mark: |
| Disabled entry recognition | :heavy_check_mark: |
| Section nesting support    | :x:                |
| Multi-line support         | :x:                |

## Installation
   Add this to your `Cargo.toml`:

   ```
   inip = "0.2.5"
   ```

## Example

```ini
; file.ini
[section]
full_name = "Hicaro"
```

```rust
use inip::parser::Parser;

fn main() {
    let parsed_file = Parser::parse("file.ini").unwrap();
    assert_eq!(parsed_file["section"]["full_name"], "Hicaro".to_string());
}
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

3. All key names must have one word

   Valid:
   ```ini
   [credentials]
   full_name = "John Doe"
   ```

   Invalid:
   ```ini
   [credentials]
   full name = "John Doe"
   ```

   If you want multiple words on your key name, use whatever style you want, but don't use space to separate that.

4. Disable entry recognition by using `;`

   ```ini
   [credentials]
   ; full_name = "John Doe"
   ```

## Contributions
If you find any problems with this library, please let me know by opening an issue explaining the problem.

## License
This project is licensed under the [MIT](LICENSE) license.
