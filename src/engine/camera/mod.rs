use nalgebra::Matrix4;
use nalgebra::Point3;

pub mod orbital_camera;

pub trait Camera {
    fn view_rotation_matrix(&self) -> Matrix4<f32>;
    fn view_matrix(&self) -> Matrix4<f32>;

    fn projection_matrix(&self) -> Matrix4<f32>;

    fn on_resize(&mut self, size: (u32, u32));
    fn on_single_update(&mut self);

    fn position(&self) -> Point3<f32>;
    fn set_position(&mut self, position: Point3<f32>);

    fn transform_model_matrix(&self, model_matrix: Matrix4<f32>) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix() * model_matrix
    }
}

