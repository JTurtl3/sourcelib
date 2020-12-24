use super::error::Error;

#[derive(Debug)]
enum Flags {
    PointSample = 0x00000001,
    TriLinear   = 0x00000002,
    ClampS      = 0x00000004,
    ClampT      = 0x00000008,
    Anisotropic = 0x00000010,
    HintDxt5    = 0x00000020,
    PwlCorrected= 0x00000040,
    Normal      = 0x00000080,
    NoMip       = 0x00000100,
    NoLOD       = 0x00000200,
    AllMips     = 0x00000400,
    Procedural  = 0x00000800,

    OneBitAlpha = 0x00001000,
    EightBitAlpha=0x00002000,
    EnvMap      = 0x00004000,
    RenderTarget= 0x00008000,
    DepthRenderTarget = 0x00010000,
    NoDebugOverride = 0x00020000,
    SingleCopy  = 0x00040000,
    PreSrgb     = 0x00080000,
    Unused0     = 0x00100000,
    Unused1     = 0x00200000,
    Unused2     = 0x00400000,
    NoDepthBuffer=0x00800000,
    NiceFilter  = 0x01000000,
    ClampU      = 0x02000000,
    VertexTexture=0x04000000,
    SSBump      = 0x08000000,
    Unused4     = 0x10000000,
    Border      = 0x20000000,
    Unused5     = 0x40000000,
    Unused6     = 0x80000000,
}

#[derive(Debug)]
enum ImageFormat {
    None = -1,
    Rgba8888 = 0,
    Abgr8888,
    Rgb888,
    Bgr888,
    Rgb565,
    I8,
    Ia88,
    P8,
    A8,
    Rgb888BlueScreen,
    Bgr888BlueScreen,
    Argb8888,
    Bgra8888,
    Dxt1,
    Dxt3,
    Dxt5,
    Bgrx8888,
    Bgr565,
    Bgrx5551,
    Bgra4444,
    Dxt1OneBitAlpha,
    Bgra5551,
    Uv88,
    Uvwq8888,
    Rgba16161616F,
    Rgba16161616,
    Uvlx8888,
}

#[derive(Debug)]
pub struct Header {
    signature: [char; 4],
    version: [u32; 2],
    header_size: u32,
    width: u16,
    height: u16,
    flags: Flags,
    frames: u16,
    first_frame: u16,
    reflectivity: [f32; 3],
    bumpmap_scale: f32,
    high_res_format: ImageFormat,
    mipmaps: u8,
    low_res_format: ImageFormat,
    low_res_width: u8,
    low_res_height: u8,
    depth: u16,
    resources: u32,
}

impl Header {
    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, Error> {
        
    }
}