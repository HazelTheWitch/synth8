use std::ops::{Deref, DerefMut, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Sample(pub f32);

impl Sample {
    pub fn to_i8(self) -> i8 {
        (self.clamp(-1.0, 1.0) * 128.5).floor() as i8
    }

    pub fn into_inner(self) -> f32 {
        self.0
    }
}

impl Deref for Sample {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Mul<f32> for Sample {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
