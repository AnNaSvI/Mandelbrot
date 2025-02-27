use std::ops::{Add, Mul};


#[derive(Clone, Copy, Debug)] 
pub struct Complex {
    pub re: f64, 
    pub im: f64, 
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    
    pub fn mag(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}




impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}



