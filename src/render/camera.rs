use cgmath::{
    perspective, InnerSpace, Matrix4, Quaternion, Rad, Rotation, Rotation3, Vector3,
};
use luminance::shader::Uniform;
use luminance::UniformInterface;

pub const CAMERA_FOVY_RAD: f32 = std::f32::consts::FRAC_PI_2;
pub const CAMERA_SENSITIVITY_STRAFE: f32 = 0.1;
pub const Z_NEAR: f32 = 0.1;
pub const Z_FAR: f32 = 10.;

pub struct CameraState {
    pub aspect_ratio: f32,
    pub fovy: f32,
    pub projection: Matrix4<f32>,
    pub cam_orient: Quaternion<f32>,
    pub cam_view: Matrix4<f32>,

    x_theta: f32,
    y_theta: f32,
    eye: Vector3<f32>,
}

impl CameraState {
    pub fn new(width: i32, height: i32) -> Self {
        let fovy = clamp_fovy(CAMERA_FOVY_RAD);
        let aspect_ratio = width as f32 / height as f32;

        let qy = Quaternion::from_angle_y(Rad(0.));
        let qx = Quaternion::from_angle_x(Rad(0.));
        let eye = Vector3::new(0., 0., 3.);

        let cam_orient = (qx * qy).normalize();
        let cam_view = Matrix4::from(cam_orient) * Matrix4::from_translation(-eye);
        let projection = perspective(Rad(fovy), aspect_ratio, Z_NEAR, Z_FAR);

        return CameraState {
            aspect_ratio: aspect_ratio,
            fovy: fovy,
            projection: projection,
            cam_orient: cam_orient,
            cam_view: cam_view,
            x_theta: 0.,
            y_theta: 0.,
            eye: eye,
        };
    }

    pub fn strafe_left(&mut self) {
        let v =
            self.cam_orient
                .invert()
                .rotate_vector(Vector3::new(CAMERA_SENSITIVITY_STRAFE, 0., 0.));
        self.eye -= v;
        self.recalc();
    }

    pub fn strafe_right(&mut self) {
        let v = self.cam_orient.invert().rotate_vector(Vector3::new(
            -CAMERA_SENSITIVITY_STRAFE,
            0.,
            0.,
        ));
        self.eye -= v;
        self.recalc();
    }

    pub fn strafe_forward(&mut self) {
        let v =
            self.cam_orient
                .invert()
                .rotate_vector(Vector3::new(0., 0., CAMERA_SENSITIVITY_STRAFE));
        self.eye -= v;
        self.recalc();
    }

    pub fn strafe_backward(&mut self) {
        let v = self.cam_orient.invert().rotate_vector(Vector3::new(
            0.,
            0.,
            -CAMERA_SENSITIVITY_STRAFE,
        ));
        self.eye -= v;
        self.recalc();
    }

    pub fn strafe_up(&mut self) {
        let v = self.cam_orient.invert().rotate_vector(Vector3::new(
            0.,
            -CAMERA_SENSITIVITY_STRAFE,
            0.,
        ));
        self.eye -= v;
        self.recalc();
    }

    pub fn strafe_down(&mut self) {
        let v =
            self.cam_orient
                .invert()
                .rotate_vector(Vector3::new(0., CAMERA_SENSITIVITY_STRAFE, 0.));
        self.eye -= v;
        self.recalc();
    }

    pub fn change_fov(&mut self, scroll: f64) {
        self.fovy += scroll as f32 * 0.1;
        self.fovy = clamp_fovy(self.fovy);

        // Because the field-of-view has changed, we need to recompute the projection matrix.
        self.projection = perspective(Rad(self.fovy), self.aspect_ratio, Z_NEAR, Z_FAR);

        self.recalc();
    }

    pub fn recalc(&mut self) {
        let qy = Quaternion::from_angle_y(Rad(self.y_theta));
        let qx = Quaternion::from_angle_x(Rad(self.x_theta));

        self.cam_orient = (qx * qy).normalize();
        self.cam_view = Matrix4::from(self.cam_orient) * Matrix4::from_translation(-self.eye);
    }

    pub fn recalc_size(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.projection = perspective(Rad(self.fovy), self.aspect_ratio, Z_NEAR, Z_FAR);
    }
}

pub fn clamp_fovy(fovy: f32) -> f32 {
    fovy.min(std::f32::consts::PI - 0.0001).max(0.0001)
}

#[derive(UniformInterface)]
pub struct CameraShaderInterface {
    #[uniform(unbound)]
    pub projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub view: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub aspect_ratio: Uniform<f32>,
}
