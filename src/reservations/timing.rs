use dashi::{Buffer, BufferInfo, BufferView, Context, Handle, MemoryVisibility, ShaderResource};
use std::time::Instant;

use super::{ReservedBinding, ReservedItem};
use crate::resolver::helpers;
#[repr(C)]
struct TimeData {
    current_time_ms: f32,
    frame_time_ms: f32,
}

pub struct ReservedTiming {
    last_time: Instant,
    buffer: Handle<Buffer>,
}

impl ReservedTiming {
    pub fn new(ctx: &mut Context) -> Self {
        let buffer = ctx
            .make_buffer(&BufferInfo {
                debug_name: "[FURIKAKE] Timing Buffer",
                byte_size: std::mem::size_of::<TimeData>() as u32,
                visibility: MemoryVisibility::CpuAndGpu,
                ..Default::default()
            })
            .expect("Unable to make timing buffer!");

        Self {
            last_time: Instant::now(),
            buffer,
        }
    }
}

impl ReservedItem for ReservedTiming {
    fn name(&self) -> String {
        "meshi_timing".to_string()
    }

    fn update(&mut self, ctx: &mut Context) {
        let s = ctx
            .map_buffer_mut::<TimeData>(self.buffer)
            .expect("Unable to map time buffer!");
        let now = std::time::Instant::now();
        s[0].current_time_ms = now.elapsed().as_secs_f32() * 1000.0;
        s[0].frame_time_ms = (now - self.last_time).as_secs_f32() * 1000.0;
        self.last_time = now;
        ctx.unmap_buffer(self.buffer)
            .expect("Unable to unmap time buffer!");
    }

    fn binding(&self) -> ReservedBinding<'_> {
        ReservedBinding::Binding(helpers::bind_group_binding(
            ShaderResource::ConstBuffer(BufferView {
                handle: self.buffer,
                size: (std::mem::size_of::<f32>() * 2) as u64,
                offset: 0,
            }),
            0,
        ))
    }
}
