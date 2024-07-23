use crate::framebuffer::Framebuffer; // Importa la estructura Framebuffer del módulo framebuffer
use crate::line::Line; // Importa el trait Line del módulo line
extern crate nalgebra_glm as glm;
use glm::Vec3; 


// Define el trait Polygon con un método draw_polygon
pub trait Polygon {
    fn draw_polygon(&mut self, points: &[Vec3]);
}
// Implementa el trait Polygon para la estructura Framebuffer
impl Polygon for Framebuffer {
    // Implementación del método draw_polygon
    fn draw_polygon(&mut self, points: &[Vec3]) {
        // Si hay menos de 3 puntos, no hay nada que dibujar. Triangulo es el menor
        if points.len() < 3 {
            return;
        }


        // Itera sobre cada punto en el array
        for i in 0..points.len() {
            // Obtiene el punto actual
            let x1 = points[i].x as usize;
            let y1 = points[i].y as usize;
            // Obtiene el siguiente punto, o el primer punto si estamos en el último punto
            let (x2, y2) = if i == points.len() - 1 {
                (points[0].x as usize, points[0].y as usize)
            } else {
                (points[i + 1].x as usize, points[i + 1].y as usize)
            };
            self.line(x1, y1, x2, y2);
        }
    }
}
