use cgmath::{Matrix4, Vector3, Point3, perspective, Rad, Deg};
use cgmath::prelude::*;
use time;

#[derive(Copy, Clone)]
pub struct SceneParams {
    pub look_at: Vector3<f32>,
    pub camera: Vector3<f32>,
    pub near: f32,
    pub far: f32,
    pub focus: f32,
}

pub struct Scene {
    // Initial params
    width: f32,
    height: f32,

    // Calculated
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    params: SceneParams,

    // Interpolated
    old_params: SceneParams,
    new_params: SceneParams,

    // Last change
    params_change_time: f64,
}

impl Scene {
    pub fn new(width: f32, height: f32, initial_params: SceneParams) -> Self {
        let mut scene = Scene {
            width,
            height,

            old_params: initial_params,
            new_params: initial_params,

            projection: Matrix4::identity(),
            view: Matrix4::identity(),
            params: initial_params,

            params_change_time: time::precise_time_s(),
        };

        scene.update();
        scene
    }

    fn interpolation(&self, mut t: f32) -> f32 {
        t *= 2.0;
        if t < 1.0 {
            0.5 * t * t
        } else {
            t -= 1.0;
            -0.5 * (t * (t - 2.0) - 1.0)
        }
    }

    pub fn update(&mut self) {
        self.params = self.calculate();

        self.projection = perspective(
            Rad::from(Deg(45.0)),
            self.width / self.height,
            self.params.near,
            self.params.far);

        self.view = Matrix4::look_at(
            Point3::from_vec(self.params.camera),
            Point3::from_vec(self.params.look_at),
            Vector3::new(0.0, 1.0, 0.0));
    }

    fn calculate(&mut self) -> SceneParams {
        let time = time::precise_time_s();
        let mut t = time - self.params_change_time;
        t /= 8.0;
        if t > 1.0 {
            self.old_params = self.new_params;
            t = 1.0;
        }
        let t = self.interpolation(t as f32);
        let look_at = self.new_params.look_at * t + self.old_params.look_at * (1.0 - t);
        let camera = self.new_params.camera * t + self.old_params.camera * (1.0 - t);
        let near = self.new_params.near * t + self.old_params.near * (1.0 - t);
        let far = self.new_params.far * t + self.old_params.far * (1.0 - t);
        let focus = self.new_params.focus * t + self.old_params.focus * (1.0 - t);
        SceneParams { look_at, camera, near, far, focus }
    }

    fn flush(&mut self) {
        let params = self.calculate();
        self.old_params = params;
    }

    pub fn set_look_at(&mut self, look_at: Vector3<f32>, animate: bool) {
        self.flush();
        self.new_params.look_at = look_at;
        if !animate {
            self.old_params.look_at = look_at;
        } else {
            self.params_change_time = time::precise_time_s();
        }
    }

    pub fn set_camera(&mut self, camera: Vector3<f32>, animate: bool) {
        self.flush();
        self.new_params.camera = camera;
        if !animate {
            self.old_params.camera = camera;
        } else {
            self.params_change_time = time::precise_time_s();
        }
    }

    pub fn set_near(&mut self, near: f32, animate: bool) {
        self.flush();
        self.new_params.near = near;
        if !animate {
            self.old_params.near = near;
        } else {
            self.params_change_time = time::precise_time_s();
        }
    }

    pub fn set_far(&mut self, far: f32, animate: bool) {
        self.flush();
        self.new_params.far = far;
        if !animate {
            self.old_params.far = far;
        } else {
            self.params_change_time = time::precise_time_s();
        }
    }

    pub fn set_focus(&mut self, focus: f32, animate: bool) {
        self.flush();
        self.new_params.focus = focus;
        if !animate {
            self.old_params.focus = focus;
        } else {
            self.params_change_time = time::precise_time_s();
        }
    }

    pub fn set_width_and_height(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_params(&self) -> SceneParams {
        self.params
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        self.view
    }

    pub fn get_projection(&self) -> Matrix4<f32> {
        self.projection
    }
}