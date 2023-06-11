use glam::{Mat4, Vec3, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::player_ship::PlayerShip,
};

use super::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub enum Frustrum {
    Perspective((f32, f32, f32, f32, f32, f32)),
    Orthogonal((f32, f32)),
}

impl Frustrum {
    pub fn orthogonal(width: f32, height: f32) -> Self {
        Frustrum::Orthogonal((width as f32, height as f32))
    }

    pub fn perspective(width: f32, height: f32, near: f32, far: f32) -> Self {
        Frustrum::Perspective((
            -width * 0.5,
            width * 0.5,
            -height * 0.5,
            height * 0.5,
            near,
            far,
        ))
    }
}

impl<'a> ComponentIteratorGenerator<'a, (&'a Transform, &'a Camera)> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = (&'a Transform, &'a Camera)> + 'a> {
        let players = self
            .iter::<PlayerShip>()
            .map(|ship| (&ship.transform, &ship.entity.camera));
        Box::new(players)
    }
}

pub struct Camera {
    projection: Mat4,
    position: Vec3,
}

pub struct CameraMatrix {
    pub projection_view_model: Mat4,
    pub view_model: Mat4,
}

impl Camera {
    pub fn new(frustrum: Frustrum, position: Vec3) -> Self {
        let projection = match frustrum {
            Frustrum::Perspective(persp) => Self::build_perspective_matrix(persp),
            Frustrum::Orthogonal(orth) => Self::build_orth_matrix(orth),
        };
        Self {
            projection,
            position,
        }
    }

    pub fn projection_view(&self, camera: &Transform) -> (Mat4, Mat4) {
        (self.projection, self.build_view_matrix(camera))
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    fn build_view_matrix(&self, camera: &Transform) -> Mat4 {
        Mat4::from_translation(-self.position)
            * Mat4::from_quat(camera.rotation.conjugate())
            * Mat4::from_translation(-camera.position)
    }

    fn build_orth_matrix((width, height): (f32, f32)) -> Mat4 {
        let mut proj = Mat4::IDENTITY;

        *proj.col_mut(0) = Vec4::new(2.0 / width, 0.0, 0.0, 0.0);
        *proj.col_mut(1) = Vec4::new(0.0, 2.0 / height, 0.0, 0.0);
        *proj.col_mut(2) = Vec4::new(0.0, 0.0, 0.0, 0.0);
        *proj.col_mut(3) = Vec4::new(1.0, 1.0, 0.0, 1.0);

        proj
    }

    fn build_perspective_matrix(
        (left, right, top, bottom, near, far): (f32, f32, f32, f32, f32, f32),
    ) -> Mat4 {
        Mat4::perspective_rh_gl(1.0, (left - right) / (top - bottom), near, far)
    }
}
