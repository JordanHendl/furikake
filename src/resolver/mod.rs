use crate::{GPUState, ReservedMetadata};
use std::collections::HashMap;

use dashi::BindGroupVariableType;

#[derive(Default, Debug)]
pub struct ResolveResult {
    pub name: String,
    pub exists: bool,
    pub binding: dashi::BindGroupVariable,
    pub set: u32,
}

pub struct Resolver {
    resolved: Vec<ResolveResult>,
}
impl Resolver {
    pub fn new<T: GPUState>(state: &T, result: &bento::CompilationResult) -> Self {
        let names = T::reserved_metadata();

        Self {
            resolved: Self::reflect_bindings(names, result).expect("Unable to create resolver!"),
        }
    }

    pub fn resolved(&self) -> &[ResolveResult] {
        self.resolved.as_slice()
    }

    fn reflect_bindings(
        names: &[ReservedMetadata],
        res: &bento::CompilationResult,
    ) -> Result<Vec<ResolveResult>, String> {
        let mut results = Vec::new();
        for b in res.variables.iter() {
            if let Some(found) = names.iter().find(|a| b.name == a.name) {
                results.push(ResolveResult {
                    name: found.name.to_string(),
                    exists: true,
                    binding: b.kind.clone(),
                    set: b.set,
                });
            } else {
                results.push(ResolveResult {
                    name: b.name.to_string(),
                    exists: false,
                    ..Default::default()
                });
            }
        }
        Ok(results)
    }
}
