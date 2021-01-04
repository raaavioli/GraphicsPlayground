use nalgebra::*;

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector3<f32>, // Yaw, Pitch, Roll
    aspect: f32,
    fov: f32,
    near_plane: f32,
    far_plane: f32,
    perspective: bool,
}

impl Camera {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, aspect: f32, fov: f32, near_plane: f32, far_plane: f32, perspective: bool) -> Self{
        Camera {
            position, rotation, fov, aspect, near_plane, far_plane, perspective,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let translation = Translation3::from(self.position);
        (translation * self.rotation_matrix()).inverse().to_homogeneous()
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn move_camera(&mut self, local_dir: Vector3<f32>, dx: f32) {
        let move_dir = local_dir.normalize();
        let rotation = self.rotation_matrix();
        self.position += rotation * move_dir * dx;
    }

    pub fn rotate_camera(&mut self, delta_rotation: Vector3<f32>) {
        self.rotation += delta_rotation;
        if self.rotation[1] > RealField::frac_pi_2() {
            self.rotation[1] = RealField::frac_pi_2();
        } else if -self.rotation[1] > RealField::frac_pi_2() {
            self.rotation[1] = RealField::frac_pi_2();
            self.rotation[1] = self.rotation[1] * -1.;
        }
    }

    fn rotation_matrix(&self) -> Rotation3<f32> {
        let yaw = Rotation3::from_axis_angle(&Vector3::y_axis(), self.rotation[0]).inverse();
        let pitch = Rotation3::from_axis_angle(&Vector3::x_axis(), self.rotation[1]).inverse();
        let roll = Rotation3::from_axis_angle(&Vector3::z_axis(), self.rotation[2]).inverse();
        return yaw * pitch * roll;
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        if self.perspective {
            Perspective3::new(self.aspect, self.fov, self.near_plane, self.far_plane).to_homogeneous()
        } else {
            Orthographic3::new(-10., 10., -10., 10., self.near_plane, self.far_plane).to_homogeneous()
        }
    }
}