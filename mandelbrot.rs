// uncomment the code below and implement the functions 

// pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
//     // your code here

//     None
// }


// pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
//     // your code here
    
// }


use crate::image::Image;
use crate::complex::Complex;
use crate::image::Pixel;

 pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
  let mut z = Complex::new(0.0,0.0);                                                   
  for it in 0..max_iterations{
    z = z * z +c;                                                                        
    if z.mag() > 4.0{                                                                       
      return Some(it);                                                                     
    }
  }

     None
 }


 pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {

    let mut image = Image::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let cx = (x as f64 / width as f64 - 0.75) * 3.5;
            let cy = (y as f64 / height as f64 - 0.5) * 2.0;
            let c = Complex::new(cx, cy);

            let pixel_color = check_pixel(c, max_iterations)
                .map(|_| Pixel::new(255, 255, 255))  //not in Mandelbrot
                .unwrap_or(Pixel::new(0, 0, 0));     //in Mandelbrot

            if let Some(pixel) = image.get_mut(x, y) {
                *pixel = pixel_color;
            }
        }
    }
    image

    
 }
