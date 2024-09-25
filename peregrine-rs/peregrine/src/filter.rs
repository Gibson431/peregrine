use core::fmt::Debug;
use nalgebra::{ComplexField, SMatrix};

#[derive(Copy, Clone, Debug)]
enum FilterError {
    FailedInverse,
}

#[derive(Clone, Copy, Debug)]
#[allow(non_snake_case)]
pub struct KalmanFilter<const R: usize, const C: usize> {
    x: SMatrix<f64, R, 1>, // State estimate vector
    p: SMatrix<f64, R, R>, // Estimate covariance
    A: SMatrix<f64, R, R>, // State transition model
    B: SMatrix<f64, R, R>, // Control input model
    h: SMatrix<f64, C, R>, // Observation model
    q: SMatrix<f64, R, R>, // Process noise covariance
    r: SMatrix<f64, C, C>, // Measurement noise covariance
    i: SMatrix<f64, R, R>, // Identity matrix
}

#[allow(non_snake_case)]
impl<const R: usize, const C: usize> KalmanFilter<R, C>
where
    f64: ComplexField,
{
    pub fn new() -> Self {
        let x = SMatrix::<f64, R, 1>::zeros();
        let p = SMatrix::<f64, R, R>::identity();
        let a = SMatrix::<f64, R, R>::identity();
        let b = SMatrix::<f64, R, R>::zeros();
        let h = SMatrix::<f64, C, R>::zeros();
        let q = SMatrix::<f64, R, R>::identity();
        let r = SMatrix::<f64, C, C>::identity();
        let i = SMatrix::<f64, R, R>::identity();
        Self {
            x,
            p,
            A: a,
            B: b,
            h,
            q,
            r,
            i,
        }
    }

    pub fn predict(&mut self, u: &SMatrix<f64, R, 1>) {
        // State prediction: x = A * x + B * u
        self.x = &self.A * &self.x + &self.B * u;

        // Covariance prediction: P = A * P * A^T + Q
        self.p = &self.A * &self.p * self.A.transpose() + &self.q;
    }
    fn update(&mut self, z: &SMatrix<f64, C, 1>) -> Result<(), FilterError> {
        // Innovation covariance: S = H * P * H^T + R
        let S = &self.h * &self.p * self.h.transpose() + &self.r;

        // Kalman gain: K = P * H^T * S^-1
        let K = &self.p * self.h.transpose() * S.try_inverse().ok_or(FilterError::FailedInverse)?;

        // Update estimate with measurement z: x = x + K * (z - H * x)
        self.x = &self.x + &K * (z - &self.h * &self.x);

        // Update the estimate covariance: P = (I - K * H) * P
        self.p = (&self.i - &K * &self.h) * &self.p;

        Ok(())
    }
}

#[embassy_executor::task]
pub async fn filter_task() {
    loop {}
}
