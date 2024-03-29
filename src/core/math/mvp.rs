use nalgebra_glm::{vec3, Mat4};
use crate::core::window::Window;

// returns an othogonal projection matrix
pub fn ortho(window: &Window) -> Mat4 {
    nalgebra_glm::ortho(0.0, window.width as f32, window.height as f32,  0.0, -1.0, 1.0)
}

// translates a matrix 
pub fn translate(src: &Mat4, x: f32, y: f32, z: f32) -> Mat4 {
    nalgebra_glm::translate(src, &vec3(x, y, z))
}

// returns a standard identity matrix
pub fn identity() -> Mat4 {
    nalgebra_glm::identity()
}

// scale a matrix
pub fn scale(src: &Mat4, x: f32, y: f32, z: f32) -> Mat4 {
    nalgebra_glm::scale(src, &vec3(x, y, z))
}

// rotate a matrix
// where xyz is the axis
pub fn rotate(src: &Mat4, angle: f32, x: f32, y: f32, z: f32) -> Mat4 {
    let axis = vec3(x, y, z);
    let axis = nalgebra_glm::normalize(&axis);
    nalgebra_glm::rotate(src, angle, &axis)
}

// get a column from a matrix
pub fn column(src: &Mat4, i: usize) -> Option<Vec<f32>> {
    if i > 3 {
        return None;
    }

    let mut column: Vec<f32> = vec![];
    
    for num in src.column(i).iter() {
        column.push(*num);
    }

    Some(column)
}

// get a row from a matrix
pub fn row(src: &Mat4, i: usize) -> Option<Vec<f32>> {
    if i > 3 {
        return None;
    }

    let mut row: Vec<f32> = vec![];
    
    for num in src.row(i).iter() {
        row.push(*num);
    }

    Some(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
        let mat = identity();
        assert_eq!(Some(vec![1.0, 0.0, 0.0, 0.0]), row(&mat, 0));
        assert_eq!(Some(vec![0.0, 0.0, 0.0, 1.0]), row(&mat, 3));
        assert_eq!(None, row(&mat, 100));
    }

    #[test]
    fn test_col() {
        let mat = identity();
        assert_eq!(Some(vec![1.0, 0.0, 0.0, 0.0]), column(&mat, 0));
        assert_eq!(Some(vec![0.0, 0.0, 0.0, 1.0]), column(&mat, 3));
        assert_eq!(None, column(&mat, 100));
    }
}