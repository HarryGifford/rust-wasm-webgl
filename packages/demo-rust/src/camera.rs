use nalgebra::{Matrix4,Vector3,Translation,UnitQuaternion};
pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;

#[derive(Debug)]
pub struct Camera {
  pos: Vec3,
  rot: UnitQuaternion<f32>,
  proj: Mat4
}

impl Camera {
  pub fn new(aspect_ratio: f32, fovy: f32, near_clip: f32, far_clip: f32) -> Camera {
    Camera {
      pos: Vector3::new(0., 0., 0.),
      rot: UnitQuaternion::face_towards(&(Vector3::z()), &Vector3::y()),
      proj: Matrix4::new_perspective(aspect_ratio, fovy, near_clip, far_clip)
    }
  }

  pub fn to_view_proj(&self) -> Mat4 {
    let xx: Mat4 = Translation::from(self.pos).into();
    let res = xx * Matrix4::from(self.rot) * self.proj;
    res
  }

  pub fn to_rot(&self) -> Mat4 {
    Matrix4::from(self.rot)
  }

  pub fn to_translation(&self) -> Mat4 {
    Translation::from(self.pos).into()
  }

  pub fn translate_from_pixels(&mut self, x: f32, y: f32) -> () {
    let dir = Vector3::new(x, y, 0.);
    let dir_pr = self.rot * dir;
    self.pos += dir_pr;
  }

}
