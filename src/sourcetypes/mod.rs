// Common data structures in the Source Engine
mod vector;

pub use vector::Vector;

// A plane dividing a map based on the BSP tree
// ...or something. Not sure, really.
pub struct Plane {
    pub normal: Vector,
    pub distance: f32,
    pub kind: i32,
}

// A pair of vertices
// defined by the vertex's index into the .bsp Vertex lump
// (LumpIndex::Vertices, or Lump #3)
pub struct Edge {
    pub v: [u16 ; 2],
}


// A geometrical face, for rendering
pub struct Face {
    pub plane_number: u16,
    pub side: u8,
    pub is_on_node: bool, // a u8/byte in the original dface_t struct
    pub first_edge: i32,
    pub num_edges: i16,
    pub tex_info: i16,
    pub disp_info: i16,
    pub surface_fog_volume_id: i16, // no idea, the wiki page's comment is "// ?"
    pub styles: [u8 ; 4],
    pub light_offset: i32,
    pub area: f32,
    // todo: shorter names, what does 'mins' mean?
    pub lightmap_texture_mins_in_luxels: [i32 ; 2],
    pub lightmap_texture_size_in_luxels: [i32 ; 2],
    pub original_face: i32,
    pub num_primitives: u16,
    pub first_primitive_id: u16,
    pub smoothing_group: u32,
}