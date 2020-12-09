use super::error::Error;

#[derive(Default, Clone, Copy, Debug)]
pub struct Lump {
    // where in the file the Lump's data is stored (byte offset)
    pub offset: u32,

    // how much data is there (in bytes)
    pub length: u32,

    // The... version.
    // Usually 0, no idea what the details are
    pub version: u32,

    // Something with Lump compression
    // I have no idea right now
    // When uncompressed, it's just [0,0,0,0]
    pub indent_code: [u8 ; 4],
}

impl Lump {
    pub fn exists(&self) -> bool {
        self.length != 0
    }

    pub fn get_data_from_bytes(&self, bytes: &Vec<u8>) -> Result<Vec<u8>, Error> {
        if self.exists() {
            if let Some(val) = bytes.get(self.offset as usize..(self.offset+self.length) as usize) {
                Ok(val.to_vec())
            } else {
                Err(Error::UnexpectedEof)
            }
        } else {
            return Ok(Vec::new())
        }
    }
}

pub const LUMPS: usize = 64;

// see https://developer.valvesoftware.com/wiki/Source_BSP_File_Format, under Lump Types
// Depending on a Lump's index in the Header's lump array,
// each lump has a specific purpose
// header.lumps[0] is Entities, lumps[1] is Planes, etc
// The order in the header lump array is unrelated to the actual order in the file
// For example, the Entities lump is index 0
// but the data is usually somewhere towards the end of the file
// Different games/engine branches can have different formats
// the differences are commented
#[derive(Debug)]
pub enum LumpIndex {
    Entities        = 0,
    Planes          = 1,
    TextureData     = 2,
    Vertices        = 3,
    Visibility      = 4,
    Nodes           = 5,
    TextureInfo     = 6,
    Faces           = 7,
    Lighting        = 8,
    Occlusion       = 9,
    Leafs           = 10,
    FaceIds         = 11, // Source 2007
    Edges           = 12,
    SurfaceEdges    = 13,
    Models          = 14,
    WorldLights     = 15,
    LeafFaces       = 16,
    LeafBrushes     = 17,
    Brushes         = 18,
    BrushSides      = 19,
    Areas           = 20,
    AreaPortals     = 21,

    // 22-25: unused in BSP Version 20+
    Portals         = 22, // UNUSED0 in 2007(TF2), PROPCOLLISION in 2009(L4D)
    Clusters        = 23, // TF2: UNUSED1, L4D: PROPHULLS
    PortalVerts     = 24, // TF2: UNUSED2, L4D: PROPHULLVERTS
    ClusterPortals  = 25, // UNUSED3, PROPTRIS

    DisplacementInfo            = 26,
    OriginalFaces               = 27,
    PhysicsDisplacement         = 28, // Source 2007
    PhysicsCollision            = 29,
    VertexNormals               = 30,
    VertexNormalIndices         = 31,
    DisplacementLightmapAlphas  = 32,
    DisplacementVertices        = 33,
    DisplacementLightmapSamplePositions = 34,

    GameLump                = 35,
    LeafWaterData           = 36,
    Primitives              = 37,
    PrimitiveVertices       = 38,
    PrimitiveIndices        = 39,
    PakFile                 = 40,
    ClipPortalVertices      = 41,
    Cubemaps                = 42,
    TextureStringData       = 43,
    TextureStringTable      = 44,
    Overlays                = 45,
    LeafDistanceToWater     = 46,
    FaceMacroTextureInfo    = 47,
    DisplacementTris        = 48,
    PhysicsCollideSurface   = 49, // Deprecated. PROP_BLOB in 2009

    // Everything after here is Source 2006+ unless specified

    WaterOverlays           = 50,
    LightMapPages           = 51, // Xbox-specific. LEAF_AMBIENT_INDEX_HDR in 2007
    LightMapPageInfo        = 52, // Xbox-specific. LEAF_AMBIENT_INDEX in 2007
    
    // 53-56: only in BSP Version 20+

    LightingHdr             = 53,
    WorldLightsHdr          = 54,
    LeafAmbientLightingHdr  = 55,
    LeafAmbientLighting     = 56,
    
    XZipPakFile             = 57, // Xbox-specific, deprecated.
    FacesHdr                = 58,
    MapFlags                = 59,

    OverlayFades            = 60, // Source 2007
    OverlaySystemLevels     = 61, // Source 2008 (Left 4 Dead)
    PhysicsLevel            = 62, // Source 2009
    DisplacementMultiblend  = 63, // Source 2010 (Alien Swarm)
}
