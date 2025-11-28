use dashi::{BindGroupVariable, Buffer, BufferView, Context, Handle, ShaderResource};

use crate::types::Camera;

use super::{ReservedBinding, ReservedItem};
use crate::resolver::helpers;

#[allow(dead_code)]
pub(crate) struct ReservedCamera {
    camera: Camera,
    buffer: Handle<Buffer>,
    variable: BindGroupVariable,
}

#[repr(C)]
#[allow(dead_code)]
struct Data {
    transform: glam::Mat4,
}

#[allow(dead_code)]
impl ReservedCamera {
    pub fn new(_ctx: &mut Context) -> Self {
        todo!()
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

impl ReservedItem for ReservedCamera {
    fn name(&self) -> String {
        "meshi_camera".to_string()
    }

    fn update(&mut self, ctx: &mut Context) {
        let s = ctx
            .map_buffer_mut::<Data>(self.buffer)
            .expect("Unable to map time buffer!");
        // update transform

        s[0].transform = self.camera.view_matrix();

        ctx.unmap_buffer(self.buffer)
            .expect("Unable to unmap time buffer!");
    }

    fn binding(&self) -> ReservedBinding<'_> {
        ReservedBinding::Binding(helpers::bind_group_binding(
            ShaderResource::ConstBuffer(BufferView {
                handle: self.buffer,
                size: (std::mem::size_of::<Data>()) as u64,
                offset: 0,
            }),
            0,
        ))
    }
}
