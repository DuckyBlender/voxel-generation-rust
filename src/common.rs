use bevy::prelude::*;

pub const RENDER_DISTANCE: i32 = 8;
pub const SEED: u32 = 2137;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const WORLD_SCALE: f64 = 0.06;
pub const NOISE_THRESHOLD: f64 = 0.3;
pub const TERRAIN_HEIGHT: i32 = 160;

// === COMPONENTS ===

#[derive(Component)]
pub struct ChunkMesh {
    pub position: IVec2XZ,
}

#[derive(Component)]
pub struct ChunkBorder;

#[derive(Component)]
pub struct UI;

// === RESOURCES ===

#[derive(Resource)]
pub struct ChunksLoaded {
    pub chunks: Vec<IVec2XZ>,
}

#[derive(Resource)]
pub struct Generating(pub bool);

#[derive(Resource, Clone)]
pub struct GameTextureAtlas(pub TextureAtlas);

// === ENUMS ===

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum BlockType {
    Bedrock,
    Stone,
    Dirt,
    Grass,
    Log,
    #[default]
    Air,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockFace {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

// === IVEC2XZ ===

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IVec2XZ {
    pub x: i32,
    pub z: i32,
}

impl IVec2XZ {
    pub fn new(x: i32, z: i32) -> Self {
        IVec2XZ { x, z }
    }

    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.z * self.z
    }
}

impl std::ops::Add for IVec2XZ {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.z + rhs.z)
    }
}

impl std::ops::Sub for IVec2XZ {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.z - rhs.z)
    }
}