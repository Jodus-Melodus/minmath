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

## Structures

### Matrix

#### Features

- Generic matrix type with const generics for size
- Operator overloading for arithmetic
- Matrix multiplication for square and non-square matrices
- Determinant calculation (2x2 only for now)
- Debug and Display formatting

The following are derived for the Matrix structure.
```rust
Clone
Copy
PartialEq
Eq
```

When a matrix is declared, its type and size (rows and columns) are specified as generic parameters. The type should implement the following traits.

```rust
Add<Output = T>
AddAssign
Sub<Output = T>
SubAssign
Mul<Output = T>
MulAssign
Div<Output = T>
DivAssign
Copy
Debug
Display
Default
```
#### Functions

```rust
fn new(data: [[T; COLUMNS]; ROWS]) -> Matrix<T, ROWS, COLUMNS>
```
- Creates a new matrix of type T and size (ROWS, COLUMNS) from the provided 2D array.

e.g
```rust
let matrix: Matrix<i32, 3, 3> = Matrix::new([
    [4, 3, 0],
    [8, 3, 9],
    [-2, 4, 2],
]);
```

```rust
fn size(&self) -> (usize, usize)
```
- Returns the size of the matrix as (rows, columns).

e.g
```rust
let matrix: Matrix<i32, 2, 3> = Matrix::new([
    [4, 3, 0],
    [8, 3, 9]
]);
let size: (usize, usize) = matrix.size();
```

```rust
fn determinant(&self) -> T
```
- Returns the determinant of the matrix (only works for 2x2 at the moment).

e.g
```rust
let matrix: Matrix<i32, 2, 2> = Matrix::new([
    [4, -3],
    [8, 3],
]);
let determinant: i32 = matrix.determinant();
```

#### Converting from Matrix to Vector

```rust
let matrix = Matrix::new([1, 2, 3]);
let vector = matrix.to_vector();
```

#### Operators

All matrix sizes are supported by the operators (square and non-square).

| Operation      | With Scalar | With Matrix |
|----------------|:-----------:|:-----------:|
| Add            | ✓ (`+`)     | ✓ (`+`)     |
| Add Assign     | ✓ (`+=`)    | ✓ (`+=`)    |
| Subtract       | ✓ (`-`)     | ✓ (`-`)     |
| Subtract Assign| ✓ (`-=`)    | ✓ (`-=`)    |
| Multiply       | ✓ (`*`)     | ✓ (`*`)     |
| Multiply Assign| ✓ (`*=`)    | ✓ (`*=`)    |
| Divide         | ✓ (`/`)     |             |
| Divide Assign  | ✓ (`/=`)    |             |

#### Matrix multiplication

```rust
let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);
let c = a * b; // c is Matrix<i32, 2, 2>
```

### Vector

#### Features

- Generic Vector type with const generics for size
- Operator overloading for arithmetic
- Debug and Display formatting

The following are derived for the Vector structure.
```rust
Clone
Copy
PartialEq
Eq
```

When a vector is declared, its type and size are specified as generic parameters. The type should implement the following traits.

```rust
Add<Output = T>
AddAssign
Sub<Output = T>
SubAssign
Mul<Output = T>
MulAssign
Div<Output = T>
DivAssign
Copy
Debug
Display
Default
```
#### Functions

```rust
pub fn new(data: [T; SIZE]) -> Self
```
- Creates a new vector of type T and size from the provided array.

e.g
```rust
let vec = Vector::new([4, -3, 2]);
```

```rust
pub fn size(&self) -> usize
```
- Returns the size of the vector.

e.g
```rust
let vec = Vector::new([4]);
        
let size: usize = vec.size();
```

#### Converting from Vector to Matrix

```rust
let vector = Vector::new([1, 2, 3]);
let matrix = vector.to_matrix();
```

#### Operators

All matrix sizes are supported by the operators (square and non-square).

| Operation      | With Scalar | With Vector |
|----------------|:-----------:|:-----------:|
| Add            | ✓ (`+`)     | ✓ (`+`)     |
| Add Assign     | ✓ (`+=`)    | ✓ (`+=`)    |
| Subtract       | ✓ (`-`)     | ✓ (`-`)     |
| Subtract Assign| ✓ (`-=`)    | ✓ (`-=`)    |
| Multiply       | ✓ (`*`)     |             |
| Multiply Assign| ✓ (`*=`)    |             |
| Divide         | ✓ (`/`)     |             |
| Divide Assign  | ✓ (`/=`)    |             |

#### Dot procuct

```rust
let vec1 = Vector::new([1, 2, 3]);
let vec2 = Vector::new([4, 5, 6]);

let dot_product = vec1.dot(vec2);
```

#### Cross procuct

The cross product is only implemented for 3D vectors.

```rust
let vec1 = Vector::new([1, 2, 3]);
let vec2 = Vector::new([4, 5, 6]);

let cross_product = vec1.cross(vec2);
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE.md) for details.

## Links

- [Crates.io](https://crates.io/crates/minmath)
- [Documentation (docs.rs)](https://docs.rs/minmath)
- [GitHub Repository](https://github.com/Jodus-Melodus/minmath)
