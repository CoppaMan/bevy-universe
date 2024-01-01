use bevy::math::{DVec3, Vec3};

pub fn vec_to_dvec3(vec: &Vec<f64>) -> DVec3 {
    DVec3 {
        x: *vec.get(0).expect(""),
        y: *vec.get(1).expect(""),
        z: *vec.get(2).expect(""),
    }
}

pub fn f32_3_to_vec3(vec: &[f32; 3]) -> Vec3 {
    Vec3 {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}
