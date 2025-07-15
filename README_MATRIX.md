# Matrix

## Features

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
## Functions

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

#### Matrix(2x2) Determinant

```rust
let matrix: Matrix<i32, 2, 2> = Matrix::new([
    [4, -3],
    [8, 3],
]);
let determinant = matrix.determinant();
```

## Operators

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

## Matrix multiplication

```rust
let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);
let c = a * b; // c is Matrix<i32, 2, 2>
```

[back](https://github.com/Jodus-Melodus/minmath/blob/master/README.md)