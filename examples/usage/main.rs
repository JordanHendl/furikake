use bento::ShaderMetadata;
use dashi::*;
use furikake::*;
use inline_spirv::*;

fn main() {
    let vert_shader_spirv: &'static [u32] = inline_spirv! {
        r#"
            #version 450 core
            layout(binding = 0) uniform meshi_timing {
                float a;
                float b;
            } meshi_timing2;

            void main() {
                gl_Position = vec4(0, 0, 0, 1);
            }
        "#,
        vert,
        no_debug,
        max_perf,
        glsl
    };
    
    let res = bento::CompilationResult {
        name: None,
        file: None,
        lang: bento::ShaderLang::Glsl,
        stage: dashi::ShaderType::Vertex,
        variables: vec![],
        metadata: ShaderMetadata {
            entry_points: todo!(),
            inputs: todo!(),
            outputs: todo!(),
            workgroup_size: todo!(),
        },
        spirv: vert_shader_spirv.to_vec(),
    };
    let mut ctx = Context::new(&Default::default()).expect("Unable to make dashi context");
    let state = DefaultState::new(&mut ctx);
    let resolver = Resolver::new(&state, &res);

    let results = resolver.resolved();
    assert!(results.len() == 1);
    
    println!("Result: {:?}", results[0]); 
}
