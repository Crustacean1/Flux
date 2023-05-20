use glam::{Mat4, Vec3, Vec4};

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

    pub fn from_size(width: f32, height: f32) -> Self {
        Frustrum::new(0.0, width as f32, 0.0, height as f32, 0.1, 10.0)
    }
}

pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,

    projection: Mat4,
    view: Mat4,
    vp_mat: Mat4,

    frustrum: Frustrum,
}

impl Camera {
    pub fn ortho(frustrum: Frustrum, pos: Vec3, dir: Vec3) -> Self {
        let mut camera = Camera {
            pos,
            dir,
            frustrum,
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            vp_mat: Mat4::IDENTITY,
        };

        camera.set_frustrum(frustrum);
        camera
    }

    pub fn persp(frustrum: Frustrum, pos: Vec3, dir: Vec3) -> Self {
        todo!()
    }

    pub fn vp_mat(&self) -> Mat4 {
        self.vp_mat
    }

    fn set_frustrum(&mut self, frustrum: Frustrum) {
        self.frustrum = frustrum;
        self.projection = self.orth_projection();
        self.vp_mat = self.projection * self.view;
    }

    pub fn ortho_from_dimensions(&mut self, (width, height): (f32, f32)) {
        self.set_frustrum(Frustrum::new(
            0.0,
            width as f32,
            0.0,
            height as f32,
            1.0,
            10.0,
        ));
    }

    fn orth_projection(&self) -> Mat4 {
        let mut proj = Mat4::IDENTITY;
        let (near, far) = (self.frustrum.near, self.frustrum.far);
        let (left, right) = (self.frustrum.left, self.frustrum.right);
        let (top, bottom) = (self.frustrum.top, self.frustrum.bottom);

        *proj.col_mut(0) = Vec4::new(2.0 / (right - left), 0.0, 0.0, 0.0);
        *proj.col_mut(1) = Vec4::new(0.0, 2.0 / (top - bottom), 0.0, 0.0);
        *proj.col_mut(2) = Vec4::new(0.0, 0.0, 2.0 / (near - far), 0.0);
        *proj.col_mut(3) = Vec4::new(
            (right + left) / (left - right),
            (top + bottom) / (bottom - top),
            0.0,
            1.0,
        );
        proj
    }

    fn persp_projection(&self) -> Mat4 {
        let mut proj = Mat4::IDENTITY;
        let (near, far) = (self.frustrum.near, self.frustrum.far);
        let (left, right) = (self.frustrum.left, self.frustrum.right);
        let (top, bottom) = (self.frustrum.top, self.frustrum.bottom);

        *proj.col_mut(0) = Vec4::new(2.0 * near / (right - left), 0.0, 0.0, 0.0);
        *proj.col_mut(1) = Vec4::new(0.0, 2.0 * near / (top - bottom), 0.0, 0.0);
        *proj.col_mut(2) = Vec4::new(
            (right + left) / (right - left),
            (top + bottom) / (top - bottom),
            (far + near) / (near - far),
            -1.0,
        );
        *proj.col_mut(3) = Vec4::new(0.0, 0.0, 2.0 * far * near / (near - far), 0.0);
        proj
    }
}
