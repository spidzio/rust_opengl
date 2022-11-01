use ultraviolet::{mat::Mat4, vec::Vec3};
use std::collections::HashMap;

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 5.0;
const SENSITIVITY: f32 = 0.05;
const ZOOM: f32 = 45.0;

#[derive(Eq, Hash, PartialEq)]
pub enum CameraMovement {
    Forward,
    Backword,
    Left,
    Right,
}

pub struct Camera {
    // Camera attributes
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,

    // Euler angles
    pub yaw: f32,
    pub pitch: f32,

    // Camera options
    pub movement_speed: f32,
    pub movement_directions: HashMap<CameraMovement, bool>,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        let front = Vec3 { x:  0.0, y:  0.0, z: -1.0 };
        let world_up = Vec3 { x:  0.0, y:  1.0, z:  0.0 };
        let right = front.cross(world_up).normalized();

        let mut camera = Self {
            position:            Vec3 { x:  0.0, y:  0.0, z:  0.0 },
            front:               front,
            up:                  Vec3 { x:  0.0, y:  1.0, z:  0.0 },
            right:               right,
            world_up:            world_up,
            yaw:                 YAW,
            pitch:               PITCH,
            movement_speed:      SPEED,
            movement_directions: HashMap::new(),
            mouse_sensitivity:   SENSITIVITY,
            zoom:                ZOOM,
        };
        camera.update_cameta_vectors();
        return camera;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard_movement(&mut self, delta_time: &f32) {
        let velocity: f32 = self.movement_speed * delta_time;

        for (direction, do_movement) in &self.movement_directions {
            if !do_movement {
                continue;
            }
            match direction {
                CameraMovement::Forward     => self.position += self.front * velocity,
                CameraMovement::Backword    => self.position -= self.front * velocity,
                CameraMovement::Left        => self.position -= self.right * velocity,
                CameraMovement::Right       => self.position += self.right * velocity,
            }
        }
    }

    pub fn start_movement(&mut self, direction: CameraMovement) {
        self.movement_directions.insert(direction, true);
    }

    pub fn stop_movement(&mut self, direction: CameraMovement) {
        self.movement_directions.insert(direction, false);
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw += x_offset * self.mouse_sensitivity;
        self.pitch += y_offset * self.mouse_sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0
        }

        self.update_cameta_vectors();
    }

    pub fn process_mouse_scroll(&mut self, offset: f32) {
        self.zoom -= offset;

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0;
        }
    }

    fn update_cameta_vectors(&mut self) {
        let front = Vec3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.front = front.normalized();
        self.right = front.cross(self.world_up).normalized();
        self.up = self.right.cross(front).normalized();
    }
}
