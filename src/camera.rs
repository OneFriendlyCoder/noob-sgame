use crate::player::*;

pub enum CameraView {
    FirstPerson,
    ThirdPerson,
}

pub struct Game {
    pub player: Player,
    pub camera_view: CameraView,
}

