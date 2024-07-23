extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod bmp;
mod line;
mod polygon;

use crate::bmp::WriteBmp;
use crate::framebuffer::{Framebuffer, Color}; 
use line::Line;
use crate::polygon::Polygon;


fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color(0xFFFFFF));
    framebuffer.clear();
    
    framebuffer.set_current_color(Color(0x000000));


    
    let points = vec![
        (165, 380)
        //prueba puntos 
    ];
    let points_vec3: Vec<Vec3> = points.iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();



    framebuffer.draw_polygon(&points_vec3);
    
    framebuffer.render_buffer("out.bmp");
        
    println!("Render de polígono 1");

}

