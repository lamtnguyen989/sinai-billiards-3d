use glam::{Vec3, DVec3};

pub const NUM_TANGENTS: usize = 6;

/***
*   Tangent vector in the phase space (linear perturbation of a trajectory point in phase space)
***/
#[derive(Clone, Copy, Default)]
pub struct TangentPhaseVector
{
    d_position: DVec3,  // Position tangent
    d_momentum: DVec3,  // Momentum tangent
}

impl TangentPhaseVector
{
    // Proper constructor
    pub fn new(pos_tangent: DVec3, mom_tangent: DVec3) -> Self {
        return Self {
            d_position: pos_tangent,
            d_momentum: mom_tangent
        }
    }

    // Constructing from array
    pub fn from_array(arr: [f64; NUM_TANGENTS]) -> Self {
        return Self {
            d_position  : DVec3::new(arr[0], arr[1], arr[2]),
            d_momentum : DVec3::new(arr[3], arr[4], arr[5])
        }
    }

    // Dot product
    pub fn dot(self, other: Self) -> f64 {
        return self.d_momentum.dot(other.d_momentum) + self.d_position .dot(other.d_position );
    }

    // Norm
    pub fn norm(self) -> f64 {
        return self.dot(self).sqrt();
    }

    // Getters
    pub fn get_position_tangent(self) -> DVec3 {return self.d_position;}
    pub fn get_momentum_tangent(self) -> DVec3 {return self.d_momentum;}
}

/// Overloading operators for arithmetics ///
// Addition 
impl std::ops::Add for TangentPhaseVector 
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self {
            d_position: self.d_position  + other.d_position,
            d_momentum: self.d_momentum + other.d_momentum
        }
    }
}

impl std::ops::AddAssign for TangentPhaseVector
{
    fn add_assign(&mut self, other: Self) {
        self.d_position += other.d_position;
        self.d_momentum += other.d_momentum;
    }
}

// Subtraction
impl std::ops::Sub for TangentPhaseVector
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self {
            d_position: self.d_position - other.d_position,
            d_momentum: self.d_momentum - other.d_momentum
        }
    }
}

impl std::ops::SubAssign for TangentPhaseVector
{
    fn sub_assign(&mut self, other: Self) {
        self.d_position -= other.d_position;
        self.d_momentum -= other.d_momentum;
    }
}

// Scalar multiplication
impl std::ops::Mul<f64> for TangentPhaseVector
{
    type Output = Self;
    fn mul(self, a: f64) -> Self {
        return Self {
            d_position: a * self.d_position,
            d_momentum: a * self.d_momentum
        }
    }
}

impl std::ops::MulAssign<f64> for TangentPhaseVector
{
    fn mul_assign(&mut self, a: f64) {
        self.d_position *= a;
        self.d_momentum *= a;
    }
}

impl std::ops::Mul<TangentPhaseVector> for f64
{
    type Output = TangentPhaseVector;
    fn mul(self, v: TangentPhaseVector) -> TangentPhaseVector {
        return TangentPhaseVector{
            d_position: v.get_position_tangent()*self,
            d_momentum: v.get_momentum_tangent()*self
        }
    }
}