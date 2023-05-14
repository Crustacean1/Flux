use glam::{Mat4, Vec3, Vec4};

#[derive(Clone, Copy)]
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
}

pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,

    projection: Mat4,
    view: Mat4,
    pv_mat: Mat4,

    frustrum: Frustrum,
}

impl Camera {
    pub fn new(frustrum: Frustrum, pos: Vec3, dir: Vec3) -> Self {
        let mut camera = Camera {
            pos,
            dir,
            frustrum,
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            pv_mat: Mat4::IDENTITY,
        };

        camera.set_frustrum(frustrum);
        println!("Camera: {:?}", camera.pv_mat);
        camera
    }

    pub fn vp_mat(&self) -> [f32; 16] {
        self.pv_mat.to_cols_array()
    }

    pub fn set_frustrum(&mut self, frustrum: Frustrum) {
        self.frustrum = frustrum;
        self.projection = self.proj();
        self.pv_mat = self.projection * self.view;
    }

    fn proj(&self) -> Mat4 {
        let mut proj = Mat4::IDENTITY;
        let (near, far) = (self.frustrum.near, self.frustrum.far);
        let (left, right) = (self.frustrum.left, self.frustrum.right);
        let (top, bottom) = (self.frustrum.top, self.frustrum.bottom);

        *proj.col_mut(0) = Vec4::new(2.0 * near / (left - right), 0.0, 0.0, 0.0);
        *proj.col_mut(1) = Vec4::new(0.0, 2.0 * near / (top - bottom), 0.0, 0.0);
        *proj.col_mut(2) = Vec4::new(0.0, 0.0, 1.0, 0.0);
        *proj.col_mut(3) = Vec4::new(0.0, 0.0, 0.0, 1.0);
        proj
    }
}
