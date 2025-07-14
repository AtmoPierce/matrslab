pub mod vector;
pub mod matrix;

#[cfg(test)]
mod tests {
    use crate::mathematics::{matrix::*, vector::*};

    #[test]
    fn test_matrix_matrix_operations(){
        let a = Matrix {
            data: [[1.0, 2.0], [3.0, 4.0]],
        };
        let b = Matrix {
            data: [[5.0, 6.0], [7.0, 8.0]],
        };
        let c = a + b; // [[6.0, 8.0], [10.0, 12.0]]
        assert_eq!(c.data, [[6.0, 8.0], [10.0, 12.0]]);
    }
    
    #[test]
    fn test_matrix_vector_operations() {
        let a = Matrix {
            data: [[1.0, 2.0], [3.0, 4.0]],
        };
        let b = Matrix {
            data: [[5.0, 6.0], [7.0, 8.0]],
        };
        let c = a + b; // [[6.0, 8.0], [10.0, 12.0]]

        assert_eq!(c.data, [[6.0, 8.0], [10.0, 12.0]]);

        let v = Vector { data: [1.0, 1.0] };
        let mv = a * v; // [1*1 + 2*1, 3*1 + 4*1] = [3.0, 7.0]

        assert_eq!(mv.data, [3.0, 7.0]);
    }
}