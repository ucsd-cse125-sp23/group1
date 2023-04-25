use nalgebra::{Vector3};
use rapier3d::prelude::*;
use std::net::{TcpStream};

pub struct PhysicsComponent {
    pub handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}
pub struct PlayerCameraComponent {
    pub camera_front: Vector3<f32>,
}

pub struct NetworkComponent {
    pub stream: TcpStream,
}