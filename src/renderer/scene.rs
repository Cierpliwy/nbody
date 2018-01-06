use cgmath::{Matrix4, Vector3, Point3, perspective, Rad, Deg};
use cgmath::prelude::*;
use time;

#[derive(Copy, Clone)]
struct SceneParams {
    look_at: Vector3<f32>,
    camera: Vector3<f32>,
}

pub struct Scene {
    pub width: f32,
    pub height: f32,
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
    old_params: SceneParams,
    new_params: SceneParams,
    params_change_time: f64,
}

impl Scene {
    pub fn new(width: f32, height: f32, look_at: Vector3<f32>, camera: Vector3<f32>) -> Self {
        let params = SceneParams {
            look_at,
            camera,
        };

        let mut scene = Scene {
            width,
            height,
            old_params: params,
            new_params: params,
            projection: Matrix4::identity(),
            view: Matrix4::identity(),
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
        let SceneParams { look_at, camera } = self.calculate();
        self.projection = perspective(Rad::from(Deg(45.0)), self.width / self.height, 1.0, 100.0);
        self.view = Matrix4::look_at(Point3::from_vec(camera), Point3::from_vec(look_at), Vector3::new(0.0, 1.0, 0.0));
    }

    fn calculate(&mut self) -> SceneParams {
        let time = time::precise_time_s();
        let mut t = time - self.params_change_time;
        if t > 1.0 {
            self.old_params = self.new_params;
            t = 1.0;
        }
        let t = self.interpolation(t as f32);
        let look_at = self.new_params.look_at * t + self.old_params.look_at * (1.0 - t);
        let camera = self.new_params.camera * t + self.old_params.camera * (1.0 - t);
        SceneParams { look_at, camera }
    }

    fn flush(&mut self) {
        let params = self.calculate();
        self.old_params = params;
        self.new_params = params;
    }

    pub fn set_look_at(&mut self, look_at: Vector3<f32>, animate: bool) {
        self.flush();
        self.new_params.look_at = look_at;
        if !animate {
            self.old_params.look_at = look_at;
        }
        self.params_change_time = time::precise_time_s();
    }

    pub fn set_camera(&mut self, camera: Vector3<f32>, animate: bool) {
        self.flush();
        self.new_params.camera = camera;
        if !animate {
            self.old_params.camera = camera;
        }
        self.params_change_time = time::precise_time_s();
    }

    pub fn set_width_and_height(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}