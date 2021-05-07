use macroquad::camera::Camera;
use macroquad::prelude as mp;

pub struct Cam {
    pub pos: mp::Vec3,
    pub uprot: f32,
    pub rightrot: f32,
}

impl Cam {
    const Z_NEAR: f32 = 0.01;
    const Z_FAR: f32 = 10000.0;
    const FOVY: f32 = 90.0;

    pub fn world_matrix(&self) -> mp::Mat4 {
        mp::Mat4::from_rotation_x(self.uprot)
            * mp::Mat4::from_rotation_y(-self.rightrot)
            * mp::Mat4::from_translation(self.pos)
    }

    pub fn translate_local(&mut self, translation: mp::Vec3) {
        self.pos += self.world_matrix().transform_vector3(translation);
    }
}

impl Camera for Cam {
    fn matrix(&self) -> mp::Mat4 {
        let aspect = mp::screen_width() / mp::screen_height();
        mp::Mat4::perspective_rh_gl(Self::FOVY, aspect, Self::Z_NEAR, Self::Z_FAR)
            * self.world_matrix()
    }

    fn depth_enabled(&self) -> bool {
        true
    }

    fn render_pass(&self) -> Option<macroquad::miniquad::RenderPass> {
        None
    }
}
