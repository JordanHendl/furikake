use glam::{Mat4, Quat, Vec3};

#[repr(C)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Camera {
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }

    /// Point the camera at a target world position.
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.position).normalize();

        // glam's look_to_rh gives a view matrix (world → camera)
        let view = Mat4::look_to_rh(self.position, forward, up);

        // Convert view → camera transform → rotation
        let world_from_camera = view.inverse();
        let (_, rot, _) = world_from_camera.to_scale_rotation_translation();

        self.rotation = rot;
    }

    /// Camera → world transform
    pub fn as_matrix(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.position)
    }

    /// View matrix (world → camera)
    pub fn view_matrix(&self) -> Mat4 {
        self.as_matrix().inverse()
    }

    /// Camera's forward (−Z in right-handed systems)
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    /// Camera's right (+X)
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Camera's up (+Y) – optional but convenient
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}
