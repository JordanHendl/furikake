use std::time::Instant;
pub mod timing;
pub mod camera;
pub use timing::*;
pub use camera::*;

use dashi::{
    BindGroupVariable, BindingInfo, Buffer, BufferView, Context, Handle, IndexedBindingInfo,
    ShaderResource,
};

pub enum ReservedBinding<'a> {
    Binding(BindingInfo),
    BindlessBinding(IndexedBindingInfo<'a>),
}

pub trait ReservedItem {
    fn name(&self) -> String;
    fn update(&mut self, ctx: &mut Context);
    fn binding(&self) -> ReservedBinding<'_>;
}

