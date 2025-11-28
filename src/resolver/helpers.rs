use std::collections::HashMap;

use crate::ReservedMetadata;

use dashi::{
    BindGroupLayoutInfo, BindGroupVariable, BindTableLayoutInfo, BindingInfo, IndexedBindingInfo,
    IndexedResource, ShaderInfo, ShaderResource, ShaderType,
};

pub const BINDLESS_SET_START: u32 = 3;

pub fn bind_group_binding(resource: ShaderResource, binding: u32) -> BindingInfo {
    BindingInfo { resource, binding }
}

pub fn bindless_binding<'a>(
    resources: &'a [IndexedResource],
    binding: u32,
) -> IndexedBindingInfo<'a> {
    IndexedBindingInfo { resources, binding }
}

#[derive(Default)]
pub struct LayoutCollection {
    pub bind_group_layouts: HashMap<u32, OwnedLayout>,
    pub bindless_layouts: HashMap<u32, OwnedLayout>,
}

impl LayoutCollection {
    pub fn bind_group_layout_info(&self, set: u32) -> Option<BorrowedBindGroupLayout<'_>> {
        self.bind_group_layouts
            .get(&set)
            .map(OwnedLayout::as_bind_group_layout)
    }

    pub fn bindless_layout_info(&self, set: u32) -> Option<BorrowedBindTableLayout<'_>> {
        self.bindless_layouts
            .get(&set)
            .map(OwnedLayout::as_bind_table_layout)
    }
}

#[derive(Default)]
pub struct OwnedLayout {
    debug_name: String,
    shaders: Vec<OwnedShaderInfo>,
}

impl OwnedLayout {
    pub fn new(set: u32) -> Self {
        Self {
            debug_name: format!("[FURIKAKE] reserved layout set {}", set),
            shaders: Vec::new(),
        }
    }

    fn push_variable(&mut self, shader_type: ShaderType, variable: BindGroupVariable) {
        if let Some(existing) = self
            .shaders
            .iter_mut()
            .find(|s| s.shader_type == shader_type)
        {
            existing.variables.push(variable);
        } else {
            self.shaders.push(OwnedShaderInfo {
                shader_type,
                variables: vec![variable],
            });
        }
    }

    fn as_bind_group_layout(&self) -> BorrowedBindGroupLayout<'_> {
        let shaders: Vec<_> = self
            .shaders
            .iter()
            .map(|s| ShaderInfo {
                shader_type: s.shader_type,
                variables: s.variables.as_slice(),
            })
            .collect();

        BorrowedBindGroupLayout {
            debug_name: &self.debug_name,
            shaders,
        }
    }

    fn as_bind_table_layout(&self) -> BorrowedBindTableLayout<'_> {
        let shaders: Vec<_> = self
            .shaders
            .iter()
            .map(|s| ShaderInfo {
                shader_type: s.shader_type,
                variables: s.variables.as_slice(),
            })
            .collect();

        BorrowedBindTableLayout {
            debug_name: &self.debug_name,
            shaders,
        }
    }
}

pub struct BorrowedBindGroupLayout<'a> {
    debug_name: &'a str,
    shaders: Vec<ShaderInfo<'a>>,
}

impl<'a> BorrowedBindGroupLayout<'a> {
    pub fn shaders(&self) -> &[ShaderInfo<'a>] {
        self.shaders.as_slice()
    }

    pub fn info(&'a self) -> BindGroupLayoutInfo<'a> {
        BindGroupLayoutInfo {
            debug_name: self.debug_name,
            shaders: self.shaders.as_slice(),
        }
    }
}

pub struct BorrowedBindTableLayout<'a> {
    debug_name: &'a str,
    shaders: Vec<ShaderInfo<'a>>,
}

impl<'a> BorrowedBindTableLayout<'a> {
    pub fn shaders(&self) -> &[ShaderInfo<'a>] {
        self.shaders.as_slice()
    }

    pub fn info(&'a self) -> BindTableLayoutInfo<'a> {
        BindTableLayoutInfo {
            debug_name: self.debug_name,
            shaders: self.shaders.as_slice(),
        }
    }
}

#[derive(Clone)]
struct OwnedShaderInfo {
    shader_type: ShaderType,
    variables: Vec<BindGroupVariable>,
}

pub fn build_layouts(
    metadata: &[ReservedMetadata],
    resolver: &super::Resolver,
    stage: ShaderType,
) -> LayoutCollection {
    let mut bind_group_layouts: HashMap<u32, OwnedLayout> = HashMap::new();
    let mut bindless_layouts: HashMap<u32, OwnedLayout> = HashMap::new();

    let meta_lookup: HashMap<&str, &ReservedMetadata> =
        metadata.iter().map(|m| (m.name, m)).collect();

    for resolved in resolver.resolved().iter().filter(|r| r.exists) {
        if let Some(meta) = meta_lookup.get(resolved.name.as_str()) {
            let var_type = resolved.binding.var_type.clone();
            debug_assert_eq!(
                var_type, meta.kind,
                "Resolved binding type does not match metadata"
            );

            let variable = BindGroupVariable {
                var_type,
                binding: resolved.binding.binding,
                count: resolved.binding.count,
            };

            let target = if resolved.set >= BINDLESS_SET_START {
                bindless_layouts
                    .entry(resolved.set)
                    .or_insert_with(|| OwnedLayout::new(resolved.set))
            } else {
                bind_group_layouts
                    .entry(resolved.set)
                    .or_insert_with(|| OwnedLayout::new(resolved.set))
            };

            target.push_variable(stage, variable);
        }
    }

    LayoutCollection {
        bind_group_layouts,
        bindless_layouts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GPUState, ReservedMetadata, resolver::Resolver};
    use dashi::BindGroupVariableType;

    struct TestState;

    impl GPUState for TestState {
        fn reserved_names() -> &'static [&'static str] {
            &["u_time", "textures"]
        }

        fn reserved_metadata() -> &'static [ReservedMetadata] {
            &[
                ReservedMetadata {
                    name: "u_time",
                    kind: BindGroupVariableType::Uniform,
                },
                ReservedMetadata {
                    name: "textures",
                    kind: BindGroupVariableType::SampledImage,
                },
            ]
        }
    }

    fn sample_compilation_result() -> bento::CompilationResult {
        bento::CompilationResult {
            name: Some("example".to_string()),
            file: Some("shader.glsl".to_string()),
            lang: bento::ShaderLang::Glsl,
            stage: ShaderType::Compute,
            variables: vec![
                bento::ShaderVariable {
                    name: "u_time".to_string(),
                    set: 0,
                    kind: BindGroupVariable {
                        var_type: BindGroupVariableType::Uniform,
                        binding: 1,
                        count: 1,
                    },
                },
                bento::ShaderVariable {
                    name: "textures".to_string(),
                    set: 3,
                    kind: BindGroupVariable {
                        var_type: BindGroupVariableType::SampledImage,
                        binding: 0,
                        count: 4,
                    },
                },
            ],
            metadata: bento::ShaderMetadata {
                entry_points: vec!["main".to_string()],
                inputs: vec![],
                outputs: vec![],
                workgroup_size: Some([1, 1, 1]),
            },
            spirv: vec![0x0723_0203, 1, 2, 3],
        }
    }

    #[test]
    fn builds_layouts_for_reserved_bindings() {
        let result = sample_compilation_result();
        let resolver = Resolver::new(&TestState, &result);

        let layouts = build_layouts(TestState::reserved_metadata(), &resolver, result.stage);

        let group_layout = layouts
            .bind_group_layout_info(0)
            .expect("expected bind group layout for set 0");
        let group_shader = group_layout.shaders().first().unwrap();
        assert_eq!(group_shader.shader_type, ShaderType::Compute);
        assert_eq!(group_shader.variables.len(), 1);
        assert_eq!(group_shader.variables[0].binding, 1);
        assert_eq!(
            group_shader.variables[0].var_type,
            BindGroupVariableType::Uniform
        );

        let table_layout = layouts
            .bindless_layout_info(3)
            .expect("expected bindless layout for set 3");
        let table_shader = table_layout.shaders().first().unwrap();
        assert_eq!(table_shader.shader_type, ShaderType::Compute);
        assert_eq!(table_shader.variables.len(), 1);
        assert_eq!(table_shader.variables[0].binding, 0);
        assert_eq!(table_shader.variables[0].count, 4);
        assert_eq!(
            table_shader.variables[0].var_type,
            BindGroupVariableType::SampledImage
        );
    }
}
