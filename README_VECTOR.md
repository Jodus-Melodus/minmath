# Vector

## Features

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
## Functions

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

## Operators

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

## Dot procuct

```rust
let vec1 = Vector::new([1, 2, 3]);
let vec2 = Vector::new([4, 5, 6]);

let dot_product = vec1.dot(vec2);
```

## Cross procuct

The cross product is only implemented for 3D vectors.

```rust
let vec1 = Vector::new([1, 2, 3]);
let vec2 = Vector::new([4, 5, 6]);

let cross_product = vec1.cross(vec2);
```

[back](https://github.com/Jodus-Melodus/minmath/blob/master/README.md)