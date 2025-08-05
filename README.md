# minmath
[![GitHub stars](https://img.shields.io/github/stars/Jodus-Melodus/minmath)](https://github.com/Jodus-Melodus/minmath/stargazers)
[![GitHub license](https://img.shields.io/github/license/Jodus-Melodus/minmath)](https://github.com/Jodus-Melodus/minmath/blob/main/LICENSE)

[![Reddit](https://img.shields.io/badge/discuss%20on-Reddit-orange?logo=reddit)](https://www.reddit.com/r/rust/comments/1lvsdvj/i_built_minmath_a_flexible_rust_math_library_to/)

[![Crates.io](https://img.shields.io/crates/v/minmath.svg)](https://crates.io/crates/minmath)
[![Crates.io downloads](https://img.shields.io/crates/d/minmath.svg)](https://crates.io/crates/minmath)
[![Docs.rs](https://docs.rs/minmath/badge.svg)](https://docs.rs/minmath)

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org/)

## Quick start example

### Matrix

```rust
use minmath::Matrix;

fn main() {
    let mat1 = Matrix::new([[1, 2], [3, 4]]);
    let mat2 = Matrix::new([[5, 6], [7, 8]]);
    let mat3 = mat1 + mat2;
    println!("{}", mat3);
}
```

### Vector

```rust
use minmath::Vector;

fn main() {
    let vec1 = Vector::new([4, -3, 0]);
    let vec2 = Vector::new([0, 3, 2]);
    let vec3 = vec1 + vec2;
    prinln!("{}", vec3);
}
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check [issues page](https://github.com/Jodus-Melodus/minmath/issues).

## Adding `minmath` to dependencies

`minmath` currently has zero dependencies and I plan to keep it that way.
There are two ways to add the crate to your dependencies.

### Manual

Add the following to your `Cargo.toml` file.

```toml
[dependencies]
minmath = "*"
# Check https://crates.io/crates/minmath for the latest version
```

### Command Line

Run the following in your terminal.

```bash
cargo add minmath
```

## Structures and Features

- [Matrix](https://github.com/Jodus-Melodus/minmath/blob/master/README_MATRIX.md)
- [Vector](https://github.com/Jodus-Melodus/minmath/blob/master/README_VECTOR.md)
- Set theory
- Probability

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE.md) for details.

## Links

- [Crates.io](https://crates.io/crates/minmath)
- [Documentation (docs.rs)](https://docs.rs/minmath)
- [GitHub Repository](https://github.com/Jodus-Melodus/minmath)
