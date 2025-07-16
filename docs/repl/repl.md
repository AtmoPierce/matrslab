# MATRSLAB Command-Line REPL

A statically-typed matrix/vector calculator in Rust  
Supports interactive calculations with MATLAB-style syntax and statically-sized matrices/vectors.

---

## Features Supported in the REPL

### **Matrix and Vector Constructors**

- **Vectors:**  
  - `[1 2 3]`, `[1, 2, 3]`
- **Matrices:**  
  - `[1 2; 3 4]`, `[1,2;3,4]`, `[1 2 3; 4 5 6; 7 8 9]`

- **Identity/Eye Matrix:**  
  - `eye(3)` or `identity(3)` → 3x3 identity matrix

- **Zeros:**  
  - `zeros(2,2)` → 2x2 matrix of zeros  
  - `zeros(3,3)`, `zeros(4,4)` supported

- **Ones:**  
  - `ones(3,3)` → 3x3 matrix of ones  
  - `ones(2,2)`, `ones(4,4)` supported

- **Diagonal Matrix:**  
  - `diag([1 2 3])` → 3x3 diagonal matrix with 1, 2, 3 on diagonal

---

### **Arithmetic Operations**

- **Vector arithmetic:**  
  - `[1 2 3] + [4 5 6]`  
  - `[1 2 3] - [4 5 6]`  
  - `[1 2 3] * 2` (scalar multiply)  
  - `[1 2 3] / 2` (scalar divide)  
  - (Note: elementwise operations; no vector dot/cross yet)

- **Matrix arithmetic:**  
  - `[1 2; 3 4] + [5 6; 7 8]`  
  - `[1 2; 3 4] * [5 6; 7 8]` (matrix-matrix multiplication)
  - `[1 2; 3 4] * 2` (scalar multiply)
  - `2 * [1 2; 3 4]` (scalar multiply)

---

### **Formatting**

- Results are shown as:
    - Vectors: `[1 2 3]`
    - Matrices: `[1 2; 3 4]` or `[1 2 3; 4 5 6; 7 8 9]`

---

### **Limits and Notes**

- Supported shapes: currently 2x2, 3x3, 4x4 for matrices (expand as needed).
- Input: Both spaces and commas allowed as separators in vectors/matrices.
- For `diag([..])`, you can use 2, 3, or 4 values (e.g., `diag([1 2])`).
- No dynamic sizing, slicing, or advanced functions yet.
- To quit: type `exit` or `quit`.

---

### **Examples**

        eye(3)
        [1 0 0; 0 1 0; 0 0 1]

        diag([1 2 3])
        [1 0 0; 0 2 0; 0 0 3]

        [1 2 3] + [4 5 6]
        [5 7 9]

        [1 2; 3 4] * [5 6; 7 8]
        [19 22; 43 50]

        zeros(2,2)
        [0 0; 0 0]


---

### **Building with the CLI**

The REPL is **optional** and built with the `cli` feature:

```sh
cargo run --features cli