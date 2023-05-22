use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

#[derive(Debug, Clone, Copy)]
pub struct Frustrum {
    near: f32,
    far: f32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Frustrum {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        Frustrum {
            left,
            right,
            top,
            bottom,
            near,
            far,
        }
    }

    pub fn ui_frustrum(width: f32, height: f32) -> Self {
        Frustrum::new(0.0, width as f32, 0.0, height as f32, 0.1, 10.0)
    }

    pub fn centered_frustrum(width: f32, height: f32, near: f32, far: f32) -> Self {
        Frustrum::new(
            -width * 0.5,
            width * 0.5,
            -height * 0.5,
            height * 0.5,
            near,
            far,
        )
    }
}

pub struct Camera {
    projection: Mat4,
    view: Mat4,
    vp_mat: Mat4,
}

impl Camera {
    pub fn new_ortho(frustrum: Frustrum) -> Self {
        let mut camera = Self::new();
        camera.orth_projection(frustrum);
        camera
    }

    pub fn new_persp(frustrum: Frustrum, pos: Vec3, dir: Vec3) -> Self {
        let mut camera = Self::new();
        camera.persp_projection(frustrum);
        camera
    }

    pub fn ortho(&mut self, frustrum: Frustrum) {
        self.orth_projection(frustrum);
    }

    pub fn persp(&mut self, frustrum: Frustrum) {
        self.persp_projection(frustrum);
    }

    pub fn view(&mut self, right: Vec3, up: Vec3, forward: Vec3, pos: Vec3) {
        //self.view = Mat4::look_to_rh(pos, dir, Vec3::new(0.0, 1.0, 0.0));
        *self.view.col_mut(0) = Vec4::new(right.x, up.x, forward.x, 0.0);
        *self.view.col_mut(1) = Vec4::new(right.y, up.y, forward.y, 0.0);
        *self.view.col_mut(2) = Vec4::new(right.z, up.z, forward.z, 0.0);
        *self.view.col_mut(3) = Vec4::new(0.0, 0.0, 0.0, 1.0);
        self.view = self.view * Mat4::from_translation(pos);
        self.vp_mat = self.projection * self.view;
    }

    pub fn pv_mat(&self) -> Mat4 {
        self.vp_mat
    }

    fn new() -> Self {
        Camera {
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            vp_mat: Mat4::IDENTITY,
        }
    }

    fn orth_projection(&mut self, frustrum: Frustrum) {
        let mut proj = Mat4::IDENTITY;
        let (near, far) = (frustrum.near, frustrum.far);
        let (left, right) = (frustrum.left, frustrum.right);
        let (top, bottom) = (frustrum.top, frustrum.bottom);

        *proj.col_mut(0) = Vec4::new(2.0 / (right - left), 0.0, 0.0, 0.0);
        *proj.col_mut(1) = Vec4::new(0.0, 2.0 / (top - bottom), 0.0, 0.0);
        *proj.col_mut(2) = Vec4::new(0.0, 0.0, 2.0 / (near - far), 0.0);
        *proj.col_mut(3) = Vec4::new(
            (right + left) / (left - right),
            (top + bottom) / (bottom - top),
            0.0,
            1.0,
        );

        self.projection = proj;
        self.vp_mat = self.projection * self.view;
    }

    fn persp_projection(&mut self, frustrum: Frustrum) {
        self.projection = Mat4::perspective_rh_gl(
            1.0,
            (frustrum.left - frustrum.right) / (frustrum.top - frustrum.bottom),
            0.1,
            100.0,
        );
        println!("Perspective: {}", self.projection);
        self.vp_mat = self.projection * self.view;
    }
}
