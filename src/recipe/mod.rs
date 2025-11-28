use bento::{BentoError, CompilationResult};
use dashi::{
    BindGroup, BindGroupLayout, BindGroupVariable, BindTable, BindTableLayout, BindingInfo, Context, Handle, IndexedResource
};

use crate::{GPUState, error::FurikakeError};

#[derive(Debug, Clone)]
pub struct BindingRecipe {
    pub binding: Option<BindingInfo>,
    pub var: bento::ShaderVariable,
}

#[derive(Debug, Clone)]
pub struct IndexedBindingRecipe {
    pub bindings: Option<Vec<IndexedResource>>,
    pub var: bento::ShaderVariable,
}

#[derive(Debug, Clone)]
pub struct BindGroupRecipe {
    pub bindings: Vec<BindingRecipe>,
    pub layout: Handle<BindGroupLayout>,
}

#[derive(Debug, Clone)]
pub struct BindTableRecipe {
    pub bindings: Vec<IndexedBindingRecipe>,
    pub layout: Handle<BindTableLayout>,
}

pub struct RecipeBook {
    bg_recipes: Vec<BindGroupRecipe>,
    bt_recipes: Vec<BindTableRecipe>,
}

impl BindingRecipe {
    pub fn cook(&mut self, ctx: &mut Context) -> Result<Handle<BindGroup>, FurikakeError> {
        // if all data in this recipe is filled out (no None bindings) then we build the bind group
        todo!()
    }
}

impl BindTableRecipe {
    pub fn cook(&mut self, ctx: &mut Context) -> Result<Handle<BindTable>, FurikakeError> {
        // if all data in this recipe is filled out (no None bindings) then we build the bind table
        todo!()
    }
}
impl RecipeBook {
    pub fn new<T: GPUState>(
        state: &T,
        shaders: &[CompilationResult],
    ) -> Result<Self, FurikakeError> {
        todo!()
    }

    pub fn recipes(&self) -> (Vec<BindGroupRecipe>, Vec<BindTableRecipe>) {
        (self.bg_recipes.clone(), self.bt_recipes.clone())
    }
}
