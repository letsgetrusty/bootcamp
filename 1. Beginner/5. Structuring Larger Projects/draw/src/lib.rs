pub fn draw_line(x: i32, y: i32) {
    // draw line without color
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;
    
    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("{color}")
        // draw line with color
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use serde::{Serialize, Deserialize};
    use rgb::RGB;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    } 
}