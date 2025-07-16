// Matrix
#[macro_export]
macro_rules! matrix {
    // Semicolon-separated rows, space or comma between columns
    ( $( $($x:expr),+ );+ $(;)? ) => {
        Matrix::from_vecs(vec![
            $(
                vec![ $($x as f64),+ ]
            ),+
        ])
    };
    // Semicolon-separated rows, space between columns
    ( $( $($x:expr)+ );+ $(;)? ) => {
        Matrix::from_vecs(vec![
            $(
                vec![ $($x as f64),+ ]
            ),+
        ])
    };
}
#[macro_export]
macro_rules! eye {
    ($n:expr) => {
        Matrix::identity($n)
    };
}

#[macro_export]
macro_rules! zeros {
    ($m:expr, $n:expr) => {
        Matrix::zeros($m, $n)
    };
}

#[macro_export]
macro_rules! ones {
    ($m:expr, $n:expr) => {
        Matrix::ones($m, $n)
    };
}

// Vectors
impl<T, const N: usize> Vector<T, N>
where
    T: Float + Default + Copy,
{
    pub fn zeros() -> Self {
        Self { data: [T::zero(); N] }
    }
    pub fn ones() -> Self {
        Self { data: [T::one(); N] }
    }
}