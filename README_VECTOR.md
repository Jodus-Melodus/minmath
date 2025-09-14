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

---

## Construction

### Creating a Vector

#### Vector2

```rust
pub fn new(x: f32, y: f32) -> Self
```

Creates a new vector2 of type `f32`.

**Example:**
```rust
let vec = Vector2::new(4.0, -3.0);
```

#### Vector3

```rust
pub fn new(x: f32, y: f32, z: f32) -> Self
```

Creates a new vector3 of type `f32`.

**Example:**
```rust
let vec = Vector3::new(4.0, -3.0, 9.0);
```

---

## Operators

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
let v1 = Vector3::new(1.0, 2.0, 3.0);
let v2 = Vector3::new(4.0, 5.0, 6.0);
let sum = v1 + v2; // [5.0, 7.0, 9.0]
let scaled = v1 * 2.0; // [2.0, 4.0, 6.0]
```

---

## Dot Product

Calculate the dot product of two vectors:

```rust
let vec1 = Vector3::new(1.0, 2.0, 3.0);
let vec2 = Vector3::new(4.0, 5.0, 6.0);
let dot_product = vec1.dot(vec2); // 32.0
```

---

## Cross Product

The cross product is only implemented for 3D vectors.

```rust
let vec1 = Vector3::new(1.0, 2.0, 3.0);
let vec2 = Vector3::new(4.0, 5.0, 6.0);
let cross_product = vec1.cross(vec2); // [-3.0, 6.0, -3.0]
```

---

## See Also

- [Matrix documentation](./README_MATRIX.md)
- [Project repository](https://github.com/Jodus-Melodus/minmath)

---
[back](https://github.com/Jodus-Melodus/minmath/blob/master/README.md)