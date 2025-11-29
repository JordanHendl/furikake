use std::ptr::NonNull;

use dashi::{
    BindGroupVariable, BindingInfo, Buffer, BufferInfo, BufferView, Context, Handle,
    IndexedBindingInfo, IndexedResource, ShaderResource,
};
use glam::Mat4;

use crate::types::Camera;

use super::{ReservedBinding, ReservedItem};

pub(crate) struct ReservedBindlessCamera {
    ctx: NonNull<Context>,
    device_camera_data: Vec<IndexedResource>,
    host_camera_data: Vec<NonNull<Camera>>,
    available: Vec<u16>,
}

impl ReservedBindlessCamera {
    pub fn new(ctx: &mut Context) -> Self {
        const START_SIZE: usize = 512;

        let mut d_data = Vec::with_capacity(START_SIZE);
        let mut h_data = Vec::with_capacity(START_SIZE);
        let available: Vec<u16> = (0..START_SIZE as u16).collect();

        for i in 0..START_SIZE {
            let default = [Mat4::IDENTITY];
            let buf = ctx
                .make_buffer(&BufferInfo {
                    debug_name: &format!("[FURIKAKE] Bindless Camera {}", i),
                    byte_size: std::mem::size_of::<Camera>() as u32,
                    visibility: dashi::MemoryVisibility::CpuAndGpu,
                    usage: dashi::BufferUsage::STORAGE,
                    initial_data: Some(unsafe { default.align_to::<u8>().1 }),
                })
                .expect("Failed making camera buffer");

            let h = ctx
                .map_buffer_mut::<Camera>(buf)
                .expect("Failed to map buffer");
            let nncam = NonNull::new(h.as_mut_ptr()).expect("NonNull failed check for camera map!");

            h_data.push(nncam);
            d_data.push(IndexedResource {
                resource: ShaderResource::StorageBuffer(buf),
                slot: i as u32,
            });
        }

        Self {
            ctx: NonNull::new(ctx).expect("NonNull failed check"),
            device_camera_data: d_data,
            host_camera_data: h_data,
            available,
        }
    }

    pub fn extend(&mut self) {
        let ctx: &mut Context = unsafe { self.ctx.as_mut() };
        if self.available.is_empty() {
            const EXTENSION_SIZE: usize = 128;
            let start = self.host_camera_data.len();
            let end = start + EXTENSION_SIZE;
            for i in start..end {
                let default = [Mat4::IDENTITY];
                let buf = ctx
                    .make_buffer(&BufferInfo {
                        debug_name: &format!("[FURIKAKE] Bindless Camera {}", i),
                        byte_size: std::mem::size_of::<Camera>() as u32,
                        visibility: dashi::MemoryVisibility::CpuAndGpu,
                        usage: dashi::BufferUsage::STORAGE,
                        initial_data: Some(unsafe { default.align_to::<u8>().1 }),
                    })
                    .expect("Failed making camera buffer");

                let h = ctx
                    .map_buffer_mut::<Camera>(buf)
                    .expect("Failed to map buffer");
                let nncam =
                    NonNull::new(h.as_mut_ptr()).expect("NonNull failed check for camera map!");

                self.host_camera_data.push(nncam);
                self.device_camera_data.push(IndexedResource {
                    resource: ShaderResource::StorageBuffer(buf),
                    slot: i as u32,
                });
            }
        }
    }

    pub fn remove_camera(&mut self, camera: Handle<Camera>) {
        if camera.valid() && (camera.slot as usize) < self.device_camera_data.len() {
            self.available.push(camera.slot);
        }
    }

    pub fn add_camera(&mut self) -> Handle<Camera> {
        if let Some(id) = self.available.pop() {
            return Handle::new(id, 0);
        } else {
            self.extend();
            return self.add_camera();
        }
    }

    pub fn camera(&self, handle: Handle<Camera>) -> &Camera {
        unsafe { self.host_camera_data[handle.slot as usize].as_ref() }
    }

    pub fn camera_mut(&mut self, handle: Handle<Camera>) -> &mut Camera {
        unsafe { self.host_camera_data[handle.slot as usize].as_mut() }
    }
}

impl ReservedItem for ReservedBindlessCamera {
    fn name(&self) -> String {
        "meshi_bindless_camera".to_string()
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), crate::error::FurikakeError> {
        todo!()
    }

    fn binding(&self) -> ReservedBinding<'_> {
        return ReservedBinding::BindlessBinding(IndexedBindingInfo {
            resources: &self.device_camera_data,
            binding: 0,
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
