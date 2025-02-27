use std::ops::{Index, IndexMut};

// Implementing the Pixel struct
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub r: u8, 
    pub g: u8, 
    pub b: u8,
}

impl Pixel {
    // Constructor for creating a new Pixel
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }
}

// Implement the Display trait for Pixel for better printing
impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

// Implementing the Image struct
pub struct Image {
    pub width: usize, 
    pub height: usize, 
    data: Vec<Pixel>,
}

impl Image {
    
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![Pixel { r: 0, g: 0, b: 0 }; width * height];
        Image { width, height, data }
    }


    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(&self.data[index])
        } else {
            None
        }
    }

    
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    
    pub fn get_mandelbrot_pixels(&self) -> usize {
        self.data.iter().filter(|&pixel| pixel.r == 0 && pixel.g == 0 && pixel.b == 0).count()
    }

   
}


impl Index<(usize, usize)> for Image {
    type Output = Pixel;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[y * self.width + x]
    }
}


impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.data[y * self.width + x]
    }
}
