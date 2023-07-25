use nalgebra::{Point3, Rotation3, Translation3, Vector3};

use super::*;

pub struct OrbitalCamera {
    pub current_position: Point3<f32>,
    pub target: Point3<f32>,
    pub distance_from_target: f32,
    pub per_update_rotation_angle_degrees: f32,

    pub fov_radians: f32,
    pub znear: f32,
    pub zfar: f32,
    pub current_projection_matrix: Matrix4<f32>,
}

impl OrbitalCamera {
    pub fn new(
        start_position: Point3<f32>,
        distance_from_target: f32,
        per_update_rotation_angle_degrees: f32,
        aspect_ratio: f32,
        fov_degrees: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let fov_radians = fov_degrees.to_radians();
        Self {
            current_position: start_position,
            target: Point3::new(0.0, 0.0, 0.0),
            distance_from_target,
            per_update_rotation_angle_degrees,

            fov_radians,
            znear,
            zfar,
            current_projection_matrix: Matrix4::new_perspective(
                aspect_ratio,
                fov_radians,
                znear,
                zfar,
            ),
        }
    }

    fn calculate_fixed_distance_vector_to_target(&self) -> Vector3<f32> {
        let normalized_vector = (self.target - self.current_position).normalize();
        normalized_vector * self.distance_from_target
    }
}

impl Camera for OrbitalCamera {
    fn view_matrix(&self) -> Matrix4<f32> {
        let target_vector = self.calculate_fixed_distance_vector_to_target();
        Matrix4::from(
            Rotation3::look_at_lh(&target_vector, &Vector3::y_axis())
                * Translation3::from(self.current_position),
        )
    }

    fn projection_matrix(&self) -> Matrix4<f32> {
        self.current_projection_matrix
    }

    fn on_resize(&mut self, size: (u32, u32)) {
        let aspect_ratio = size.0 as f32 / size.1 as f32;
        self.current_projection_matrix =
            Matrix4::new_perspective(aspect_ratio, self.fov_radians, self.znear, self.zfar);
    }

    fn on_single_update(&mut self) {
        let target_to_camera_vector = -self.calculate_fixed_distance_vector_to_target();
        let rotated_origin_camera_vector =
            Rotation3::from_axis_angle(&Vector3::y_axis(), self.per_update_rotation_angle_degrees.to_radians())
                .transform_vector(&target_to_camera_vector);
        self.current_position = self.target + rotated_origin_camera_vector;
    }

    fn set_position(&mut self, position: Point3<f32>) {
        self.current_position = position;
    }
}
