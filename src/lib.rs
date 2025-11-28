pub mod error;
pub mod reservations;
pub mod resolver;
pub mod types;


use dashi::{BindGroupVariableType, Context};
use error::FurikakeError;
use reservations::{ReservedItem, ReservedTiming};
use std::{collections::HashMap, ptr::NonNull};

pub use resolver::*;

pub struct ReservedMetadata {
    pub name: &'static str,
    pub kind: BindGroupVariableType,
}

pub trait GPUState {
    fn reserved_names() -> &'static [&'static str];
    fn reserved_metadata() -> &'static [ReservedMetadata];
}

pub struct DefaultState {
    ctx: NonNull<Context>,
    reserved: HashMap<String, Box<dyn ReservedItem>>,
}

pub struct BindlessState {
    ctx: NonNull<Context>,
    reserved: HashMap<String, Box<dyn ReservedItem>>,
}

///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///

const DEFAULT_STATE_NAMES: [&str; 1] = ["meshi_timing"];
const DEFAULT_METADATA: [ReservedMetadata; 1] = [ReservedMetadata {
    name: "meshi_timing",
    kind: BindGroupVariableType::Uniform,
}];

impl GPUState for DefaultState {
    fn reserved_names() -> &'static [&'static str] {
        DEFAULT_STATE_NAMES.as_slice()
    }

    fn reserved_metadata() -> &'static [ReservedMetadata] {
        DEFAULT_METADATA.as_slice()
    }
}

impl DefaultState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut reserved: HashMap<String, Box<dyn ReservedItem>> = HashMap::new();

        let names = DEFAULT_STATE_NAMES;
        reserved.insert(names[0].to_string(), Box::new(ReservedTiming::new(ctx)));

        Self {
            reserved,
            ctx: NonNull::from_ref(ctx),
        }
    }

    pub fn binding(&self, key: &str) -> Result<&dyn ReservedItem, FurikakeError> {
        if let Some(b) = self.reserved.get(key) {
            return Ok(b.as_ref());
        }

        Err(FurikakeError {})
    }

    pub fn update(&mut self) {
        let ctx: &mut Context = unsafe { self.ctx.as_mut() };
        for iter in &mut self.reserved {
            iter.1.update(ctx);
        }
    }
}

///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///

const BINDLESS_STATE_NAMES: [&str; 1] = ["meshi_timing"];
const BINDLESS_METADATA: [ReservedMetadata; 1] = [ReservedMetadata {
    name: "meshi_timing",
    kind: BindGroupVariableType::Uniform,
}];

impl GPUState for BindlessState {
    fn reserved_names() -> &'static [&'static str] {
        BINDLESS_STATE_NAMES.as_slice()
    }

    fn reserved_metadata() -> &'static [ReservedMetadata] {
        BINDLESS_METADATA.as_slice()
    }

}

impl BindlessState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut reserved: HashMap<String, Box<dyn ReservedItem>> = HashMap::new();

        let names = BINDLESS_STATE_NAMES;
        reserved.insert(names[0].to_string(), Box::new(ReservedTiming::new(ctx)));

        Self {
            reserved,
            ctx: NonNull::from_ref(ctx),
        }
    }

    pub fn binding(&self, key: &str) -> Result<&dyn ReservedItem, FurikakeError> {
        if let Some(b) = self.reserved.get(key) {
            return Ok(b.as_ref());
        }

        Err(FurikakeError {})
    }

    pub fn update(&mut self) {
        let ctx: &mut Context = unsafe { self.ctx.as_mut() };
        for iter in &mut self.reserved {
            iter.1.update(ctx);
        }
    }
}
