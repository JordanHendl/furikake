# Usage example

This example demonstrates how to reflect reserved bindings with `furikake`,
compile inline GLSL shaders, and render a simple quad using Dashi.

## Running

```bash
cargo run --example usage
```

The program draws offscreen; no window will appear.

## Bindful (DefaultState) example

`DefaultState` owns the bindful reservations and currently exposes a single
reserved item: the timing uniform (`meshi_timing`). The sample program in
`main.rs` drives that binding end to end by compiling shaders, validating the
reserved binding, and drawing a quad using the bind group recipe produced by
`furikake`.

```rust
let mut ctx = Context::headless(&ContextInfo::default())?;
let mut state = DefaultState::new(&mut ctx);

// Reflect the timing uniform in both shaders.
let vert_resolver = Resolver::new(&state, &vertex_shader)?;
let frag_resolver = Resolver::new(&state, &fragment_shader)?;

println!("validated timing: {:?} / {:?}", vert_resolver.resolved(), frag_resolver.resolved());

// Build bind group layouts and bind groups from the recipe book so that the
// `meshi_timing` uniform gets bound at set 0 / binding 0 automatically.
let book = RecipeBook::new(&mut ctx, &state, &[vertex_shader, fragment_shader])?;
let (mut bg_recipes, _) = book.recipes();

let timing_bind_group = bg_recipes
    .drain(..)
    .find_map(|recipe| recipe.cook(&mut ctx).ok())
    .expect("timing bind group");

// Refresh the timing buffer before issuing draw commands.
state.update()?;

// Bind the timing group when recording commands.
draw.draw_indexed(&DrawIndexed {
    bind_groups: [Some(timing_bind_group), None, None, None],
    ..Default::default()
});
```

Running `cargo run --example usage` prints confirmation that both shaders
validated the `meshi_timing` uniform and that the draw completed successfully.

## Bindless (BindlessState) example

`BindlessState` exposes all reserved items, including the bindless tables for
cameras, textures, transformations, and materials. The snippet below shows how
to allocate entries for each bindless buffer, write host-side data, and validate
all reserved bindings in a shader pack. Each reserved item is bound at set 0
with the binding indices shown in comments.

```rust
use furikake::reservations::{
    bindless_camera::ReservedBindlessCamera,
    bindless_materials::ReservedBindlessMaterials,
    bindless_textures::ReservedBindlessTextures,
    bindless_transformations::ReservedBindlessTransformations,
};

let mut ctx = Context::headless(&ContextInfo::default())?;
let mut state = BindlessState::new(&mut ctx);

// Allocate one entry in every bindless buffer and mutate the host-side structs.
state.reserved_mut::<ReservedBindlessCamera, _>("meshi_bindless_camera", |cameras| {
    let camera = cameras.add_camera();
    let cam = cameras.camera_mut(camera);
    cam.position = glam::Vec3::new(0.0, 2.0, 5.0);
});

state.reserved_mut::<ReservedBindlessTextures, _>("meshi_bindless_textures", |textures| {
    let tex = textures.add_texture();
    let t = textures.texture_mut(tex);
    t.id = 7; // binding 0 in set 0
});

state.reserved_mut::<ReservedBindlessTransformations, _>(
    "meshi_bindless_transformations",
    |xforms| {
        let xf = xforms.add_transformation();
        let m = xforms.transform_mut(xf);
        m.model[0][3] = 1.0; // binding 0 in set 0
    },
);

state.reserved_mut::<ReservedBindlessMaterials, _>("meshi_bindless_materials", |materials| {
    let mat = materials.add_material();
    let m = materials.material_mut(mat);
    m.base_color = [0.8, 0.7, 0.6, 1.0]; // binding 0 in set 0
});

// Timing remains available in the bindless state as well.
state.reserved_mut::<ReservedTiming, _>("meshi_timing", |_timing| {
    // Updated automatically in `state.update()`.
});

// Reflect and build bind groups for all reserved items at once.
let shaders = load_bindless_shader_pack();
let book = RecipeBook::new(&mut ctx, &state, &shaders)?;
let (mut bg_recipes, _) = book.recipes();

let bind_groups: Vec<_> = bg_recipes
    .drain(..)
    .map(|recipe| recipe.cook(&mut ctx))
    .collect::<Result<_, _>>()?;

state.update()?; // uploads timing and any edited bindless data

// Bind group order: [timing, camera, textures, transformations, materials]
draw.draw_indexed(&DrawIndexed {
    bind_groups: bind_groups.try_into().unwrap_or_default(),
    ..Default::default()
});
```

The bindless example validates and binds every reserved item:

- `meshi_timing` — uniform buffer at set 0, binding 0
- `meshi_bindless_camera` — bindless buffer at set 0, binding 0
- `meshi_bindless_textures` — bindless buffer at set 0, binding 0
- `meshi_bindless_transformations` — bindless buffer at set 0, binding 0
- `meshi_bindless_materials` — bindless buffer at set 0, binding 0

Each buffer exposes `add_*`/`remove_*` and `*_mut` helpers so host code can
write into the mapped memory before calling `state.update()` to keep GPU-visible
storage in sync.
