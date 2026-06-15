use nalgebra::{
    SMatrix, Const, DimMin, DimDiff, DimSub,
    DefaultAllocator, allocator::Allocator,
    ArrayStorage, U1
};

/***
*  Lyapunov spectra computation handler
***/

// Enum to denote how the frame which is a matrix will be laid out in memory
pub enum FrameLayout {
    ColumnMajor, 
    RowMajor
}

// Actual Lyapunov spectra computation handler
#[derive(Clone)]
pub struct LyapunovSpectra<const N: usize>
{
    frame:      SMatrix<f64, N, N>,
    spectrum:   [f64; N]
}

impl<const N: usize> LyapunovSpectra<N> 
where 
    Const<N>: DimMin<Const<N>, Output = Const<N>>
            + DimSub<U1>,
    DimDiff<Const<N>, U1>: nalgebra::DimMin<Const<N>>,
    DefaultAllocator: Allocator<Const<N>, Const<N>, Buffer<f64> = ArrayStorage<f64, N, N>> + Allocator<Const<N>>
                    + Allocator<Const<N>>
                    + Allocator<DimDiff<Const<N>, U1>>
                    + Allocator<Const<N>, DimDiff<Const<N>, U1>>
                    + Allocator<DimDiff<Const<N>, U1>, Const<N>>, 
{
    // Constructor
    pub fn new() -> Self {
        return Self {
            frame:      SMatrix::identity(),
            spectrum:   [0.0; N]
        }
    }

    
    // Storing contents from array slice
    pub fn frame_from_slice(&mut self, data: &[f64], frame_layout: FrameLayout) {
        // I will need to make this a Result<_, _> later on for more comprehensive error handling
        assert_eq!(data.len(), N*N, "Incompatible slice to build frame");

        // Storing data
        self.frame = match frame_layout {
            FrameLayout::ColumnMajor => SMatrix::from_column_slice(data),
            FrameLayout::RowMajor    => SMatrix::from_row_slice(data),
        }
    }
    
    // Reorthorgonalize frame for improving stability and correctness of the solution spectra (QR-decomposition default)
    #[inline]
    pub fn reorthorgonalize_frame(&mut self) {
        let frame_qr_decomp = self.frame.clone().qr();
        self.frame.copy_from(&frame_qr_decomp.q());
    }

    // Compute the Lyapunov spectra using QR-decomposition of internal frame (QR-decomposition default)
    #[inline]
    pub fn compute_from_frame(&mut self, t: f64, total_time: f64) -> () {
        // Take QR-decomposition of the frame
        let frame_qr_decomp = self.frame.clone().qr();

        // Update the frame as the Q-matrix (re-orthonormalize for numerical stability)
        self.frame.copy_from(&frame_qr_decomp.q());

        // Calculate the Lyapunov exponents increments as the natural log of the diagonals of R-matrix (singular values approximation)
        let r_matrix = frame_qr_decomp.r();
        let increments: [f64; N] = std::array::from_fn(|k| {f64::ln(r_matrix[(k,k)].abs().max(1e-16))});

        // Update the spectra and phase frame based on computed increments
        for k in 0..N {self.spectrum[k] += (increments[k] - self.spectrum[k]*t) / total_time;}
    }

    // Compute the Lyapunov spectra using SVD-decomposition of internal frame (SVD version)
    #[inline]
    pub fn compute_from_frame_svd(&mut self, t: f64, total_time: f64)  -> () {
        // SVD decomposition of the frame 
        let frame_svd = self.frame.clone().svd(true, true);

        todo!("Apply Greene & Kim methods");
    }

    // Getters
    pub fn get_spectrum(&self) -> [f64; N] {return self.spectrum;}
    pub fn get_frame_mut(&mut self) -> &mut SMatrix<f64, N, N> {return &mut self.frame;}
    pub fn get_frame(&self) -> SMatrix<f64, N, N> {return self.frame;}
} 
