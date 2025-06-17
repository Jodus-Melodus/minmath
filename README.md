# minmath

- [Crates.io](https://crates.io/crates/minmath)
- [GitHub](https://github.com/Jodus-Melodus/minmath)

## Quick start example

```rust
use minmath::Matrix;

fn main() {
    let a = Matrix::new([[1, 2], [3, 4]]);
    let b = Matrix::new([[5, 6], [7, 8]]);
    let c = a + b;
    println!("{:?}", c);
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

The following are derived for the Matrix structure.
```rust
Clone
Copy
PartialEq
Eq
```

When a matrix is declared the type and size (rows and columns) are implicitly derived. The type should implement the following traits.

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
Default
```
#### Functions

```rust
fn new(data: [[T; COLUMNS]; ROWS]) -> Matrix
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
fn size() -> (usize, usize)
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
fn determinant() -> T
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

#### Other implementations
```rust
Debug
Display
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.