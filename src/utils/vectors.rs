use bevy::math::DVec3;

pub fn vec_to_dvec3(vec: &Vec<f64>) -> DVec3 {
    DVec3 {
        x: *vec.get(0).expect(""),
        y: *vec.get(1).expect(""),
        z: *vec.get(2).expect(""),
    }
}
