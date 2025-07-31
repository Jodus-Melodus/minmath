# Vector

## Features

- **Generic Vector type** with const generics for size
- **Operator overloading** for arithmetic with scalars and other vectors
- **Debug** and **Display** formatting for easy printing
- **Conversion** between vectors and matrices
- **Dot and cross products**
- **Rotation** of 2D and 3D vectors using rotation matrices

---

## Traits

The following traits are derived for the `Vector` structure:

```rust
Clone
Copy
PartialEq
Eq
```

To use all features, the vector's element type `T` should implement:

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

---

## Construction

### Creating a Vector

```rust
pub fn new(data: [T; SIZE]) -> Self
```

Creates a new vector of type `T` and size `SIZE` from the provided array.

**Example:**
```rust
let vec = Vector::new([4, -3, 2]);
```

### Getting the Size

```rust
pub fn size(&self) -> usize
```

Returns the size of the vector.

**Example:**
```rust
let vec = Vector::new([4]);
let size: usize = vec.size(); // 1
```

---

## Conversion

### Vector to Matrix

Convert a vector to a column matrix:

```rust
let vector = Vector::new([1, 2, 3]);
let matrix = vector.to_matrix();
```

---

## Operators

All matrix sizes are supported by the operators (square and non-square).

| Operation         | With Scalar | With Vector |
|-------------------|:-----------:|:-----------:|
| Add (`+`)         | ✓           | ✓           |
| Add Assign (`+=`) | ✓           | ✓           |
| Subtract (`-`)    | ✓           | ✓           |
| Sub Assign (`-=`) | ✓           | ✓           |
| Multiply (`*`)    | ✓           |             |
| Mul Assign (`*=`) | ✓           |             |
| Divide (`/`)      | ✓           |             |
| Div Assign (`/=`) | ✓           |             |

**Example:**
```rust
let v1 = Vector::new([1, 2, 3]);
let v2 = Vector::new([4, 5, 6]);
let sum = v1 + v2; // [5, 7, 9]
let scaled = v1 * 2; // [2, 4, 6]
```

---

## Dot Product

Calculate the dot product of two vectors:

```rust
let vec1 = Vector::new([1, 2, 3]);
let vec2 = Vector::new([4, 5, 6]);
let dot_product = vec1.dot(vec2); // 32
```

---

## Cross Product

The cross product is only implemented for 3D vectors.

```rust
let vec1 = Vector::new([1, 2, 3]);
let vec2 = Vector::new([4, 5, 6]);
let cross_product = vec1.cross(vec2); // [-3, 6, -3]
```

---

## Rotating Vectors

You can rotate 2D and 3D vectors using the provided rotation matrix constructors and the `to_matrix()` and `to_vector()` conversion methods.

### 2D Rotation

To rotate a 2D vector by an angle θ (in radians), use `Matrix::rotation_matrix2x2`:

```rust
use std::f32::consts::FRAC_PI_2; // 90 degrees

let v = Vector::new([1.0_f32, 0.0]);
let m = v.to_matrix();
let rotated = Matrix::<f32, 2, 2>::rotation_matrix2x2(FRAC_PI_2) * m;
let result = rotated.to_vector(); // result is approximately [0.0, 1.0]
```

#### More 2D Examples

- **180° rotation:**
  ```rust
  use std::f32::consts::PI;
  let v = Vector::new([1.0, 0.0]);
  let m = v.to_matrix();
  let rotated = Matrix::<f32, 2, 2>::rotation_matrix2x2(PI) * m;
  let result = rotated.to_vector(); // [-1.0, 0.0]
  ```

- **45° rotation:**
  ```rust
  use std::f32::consts::FRAC_PI_4;
  let v = Vector::new([1.0, 0.0]);
  let m = v.to_matrix();
  let rotated = Matrix::<f32, 2, 2>::rotation_matrix2x2(FRAC_PI_4) * m;
  let result = rotated.to_vector(); // [sqrt(2)/2, sqrt(2)/2]
  ```

### 3D Rotation

To rotate a 3D vector about the X, Y, or Z axis by an angle θ (in radians), use the corresponding rotation matrix:

#### About the X axis

```rust
use std::f32::consts::FRAC_PI_2; // 90 degrees

let v = Vector::new([1.0, 2.0, 3.0]);
let m = v.to_matrix();
let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_x(FRAC_PI_2) * m;
let result = rotated.to_vector(); // [1.0, -3.0, 2.0]
```

#### About the Y axis

```rust
use std::f32::consts::FRAC_PI_2; // 90 degrees

let v = Vector::new([1.0, 2.0, 3.0]);
let m = v.to_matrix();
let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_y(FRAC_PI_2) * m;
let result = rotated.to_vector(); // [3.0, 2.0, -1.0]
```

#### About the Z axis

```rust
use std::f32::consts::FRAC_PI_2; // 90 degrees

let v = Vector::new([1.0, 2.0, 3.0]);
let m = v.to_matrix();
let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_z(FRAC_PI_2) * m;
let result = rotated.to_vector(); // [-2.0, 1.0, 3.0]
```

---

## Notes

- All angles are in radians.
- For 2D, use `rotation_matrix2x2`. For 3D, use the axis-specific rotation matrix.
- The result of a rotation is a new `Vector` containing the rotated coordinates.

---

## See Also

- [Matrix documentation](./README_MATRIX.md)
- [Project repository](https://github.com/Jodus-Melodus/minmath)

---
[back](https://github.com/Jodus-Melodus/minmath/blob/master/README.md)