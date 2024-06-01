use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

const FPS: u64 = 50;
pub const DELTA: f32 = 1.0 / FPS as f32;

#[derive(Pod, Clone, Copy, Debug, Zeroable)]
#[repr(C)]
pub struct RobotState {
    pub origin: [f32; 2],
    pub velocity: [f32; 2],
    pub rotation: f32,
    pub bucket_height: f32,
    pub bucket_angle: f32,
}

impl std::fmt::Display for RobotState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "origin: [{:.2}, {:.2}], velocity: [{:.2}, {:.2}], rotation: {:.2}, bucket_height: {:.2}, bucket_angle: {:.2}",
            self.origin[0], self.origin[1],
            self.velocity[0], self.velocity[1],
            self.rotation,
            self.bucket_height,
            self.bucket_angle,
        )
    }
}

impl RobotState {
    pub const fn new() -> Self {
        Self {
            origin: [0.0, 0.0],
            velocity: [0.0, 0.0],
            rotation: 0.0,
            bucket_height: 0.1,
            bucket_angle: 0.0,
        }
    }
}

impl Default for RobotState {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Action {
    SetDrive(f32, f32),
    SetBucketHeight(f32),
    SetBucketAngle(f32),
}