pub mod camera;
pub mod timing;
pub use timing::*;

use dashi::{BindingInfo, Context, IndexedBindingInfo};

pub enum ReservedBinding<'a> {
    Binding(BindingInfo),
    BindlessBinding(IndexedBindingInfo<'a>),
}

pub trait ReservedItem {
    fn name(&self) -> String;
    fn update(&mut self, ctx: &mut Context);
    fn binding(&self) -> ReservedBinding<'_>;
}
