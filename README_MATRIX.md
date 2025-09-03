# Matrix

## Features

- **Generic matrix type** with const generics for rows and columns
- **Operator overloading** for arithmetic with scalars and other matrices
- **Matrix multiplication** for both square and non-square matrices
- **Determinant calculation** (currently for 2x2 matrices)
- **Debug** and **Display** formatting for easy inspection

---

## Traits

The following traits are derived for the `Matrix` structure:

```rust
Clone
Copy
PartialEq
Eq
```

---

## Construction

### Creating a Matrix

```rust
fn new(data: [[f32; COLUMNS]; ROWS]) -> Matrix<ROWS, COLUMNS>
```

Creates a new matrix of type `f32` and size `(ROWS, COLUMNS)` from the provided 2D array.

**Example:**
```rust
let matrix: Matrix<3, 3> = Matrix::new([
    [4.0, 3.0, 0.0],
    [8.0, 3.0, 9.0],
    [-2.0, 4.0, 2.0],
]);
```

---

## Rotation Matrices

### 2D Rotation Matrix

```rust
pub fn rotation_matrix2x2(theta: f32) -> Matrix<2, 2>
```
Creates a 2D rotation matrix for angle `theta` (in radians):

```rust
let rot = Matrix::<2, 2>::rotation_matrix2x2(std::f32::consts::FRAC_PI_2);
// Rotates vectors by 90 degrees counterclockwise
```

### 3D Rotation Matrices

- **About the X axis:**
  ```rust
  pub fn rotation_matrix3x3_x(theta: f32) -> Matrix<3, 3>
  ```
- **About the Y axis:**
  ```rust
  pub fn rotation_matrix3x3_y(theta: f32) -> Matrix<3, 3>
  ```
- **About the Z axis:**
  ```rust
  pub fn rotation_matrix3x3_z(theta: f32) -> Matrix<3, 3>
  ```

**Example:**
```rust
let rot_x = Matrix::<3, 3>::rotation_matrix3x3_x(std::f32::consts::FRAC_PI_2);
let rot_y = Matrix::<3, 3>::rotation_matrix3x3_y(std::f32::consts::FRAC_PI_2);
let rot_z = Matrix::<3, 3>::rotation_matrix3x3_z(std::f32::consts::FRAC_PI_2);
```

---

## Methods

### Getting the Size

```rust
fn size(&self) -> (usize, usize)
```
Returns the size of the matrix as `(rows, columns)`.

**Example:**
```rust
let matrix: Matrix<2, 3> = Matrix::new([
    [4.0, 3.0, 0.0],
    [8.0, 3.0, 9.0]
]);
let size = matrix.size(); // (2, 3)
```

### Determinant

```rust
fn determinant(&self) -> T
```
Returns the determinant of the matrix (currently only for 2x2 matrices).

**Example:**
```rust
let matrix: Matrix<2, 2> = Matrix::new([
    [4.0, -3.0],
    [8.0, 3.0],
]);
let determinant = matrix.determinant(); // -36.0
```

### Converting from Matrix to Vector

Convert a column matrix to a vector:

```rust
let matrix = Matrix::new([[1.0], [2.0], [3.0]]);
let vector = matrix.to_vector();
```

---

## Operators

All matrix sizes are supported by the operators (square and non-square).

| Operation         | With Scalar | With Matrix |
|-------------------|:-----------:|:-----------:|
| Add (`+`)         | ✓           | ✓           |
| Add Assign (`+=`) | ✓           | ✓           |
| Subtract (`-`)    | ✓           | ✓           |
| Sub Assign (`-=`) | ✓           | ✓           |
| Multiply (`*`)    | ✓           | ✓           |
| Mul Assign (`*=`) | ✓           | ✓           |
| Divide (`/`)      | ✓           |             |
| Div Assign (`/=`) | ✓           |             |

**Example:**
```rust
let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
let b = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);
let sum = a + b; // [[6.0, 8.0], [10.0, 12.0]]
let scaled = a * 2.0; // [[2.0, 4.0], [6.0, 8.0]]
```

---

## Matrix Multiplication

Matrix multiplication is supported for compatible sizes (inner dimensions must match):

```rust
let a = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let b = Matrix::new([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
let c = a * b; // c is Matrix<2, 2>
```

---

## Notes

- All rotation angles are in radians.
- Matrix multiplication follows standard linear algebra rules.
- The result of multiplying two matrices is a new `Matrix` with the appropriate dimensions.

---

## See Also

- [Vector documentation](./README_VECTOR.md)