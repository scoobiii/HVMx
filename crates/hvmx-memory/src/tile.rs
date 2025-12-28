// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: tile.rs
// Location: crates/hvmx-memory/src/tile.rs
// Purpose: Tiled memory layout for GPU efficiency
// Authors: scoobiii & GOS3 (Gang of Seven Senior Scrum LLM DevOps Team)
// Date: 2024-12-28
// License: MIT OR Apache-2.0
// ==============================================================================

/// Tile configuration for memory layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileConfig {
    pub width: usize,
    pub height: usize,
}

impl TileConfig {
    /// Common tile sizes for mobile GPUs
    pub const TILE_8X8: TileConfig = TileConfig { width: 8, height: 8 };
    pub const TILE_16X16: TileConfig = TileConfig { width: 16, height: 16 };
    pub const TILE_32X32: TileConfig = TileConfig { width: 32, height: 32 };

    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

/// Convert linear index to tiled coordinates
pub fn linear_to_tiled(index: usize, tile: TileConfig, total_width: usize) -> (usize, usize) {
    let tiles_per_row = total_width / tile.width;
    let tile_index = index / tile.area();
    let within_tile = index % tile.area();
    
    let tile_row = tile_index / tiles_per_row;
    let tile_col = tile_index % tiles_per_row;
    
    let local_row = within_tile / tile.width;
    let local_col = within_tile % tile.width;
    
    let x = tile_col * tile.width + local_col;
    let y = tile_row * tile.height + local_row;
    
    (x, y)
}

/// Convert tiled coordinates to linear index
pub fn tiled_to_linear(x: usize, y: usize, tile: TileConfig, total_width: usize) -> usize {
    let tiles_per_row = total_width / tile.width;
    
    let tile_col = x / tile.width;
    let tile_row = y / tile.height;
    
    let local_col = x % tile.width;
    let local_row = y % tile.height;
    
    let tile_index = tile_row * tiles_per_row + tile_col;
    let within_tile = local_row * tile.width + local_col;
    
    tile_index * tile.area() + within_tile
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_config() {
        let tile = TileConfig::new(16, 16);
        assert_eq!(tile.width, 16);
        assert_eq!(tile.height, 16);
        assert_eq!(tile.area(), 256);
        assert!(tile.is_square());
    }

    #[test]
    fn test_tile_presets() {
        assert_eq!(TileConfig::TILE_8X8.area(), 64);
        assert_eq!(TileConfig::TILE_16X16.area(), 256);
        assert_eq!(TileConfig::TILE_32X32.area(), 1024);
    }

    #[test]
    fn test_linear_to_tiled() {
        let tile = TileConfig::TILE_8X8;
        let (x, y) = linear_to_tiled(0, tile, 64);
        assert_eq!((x, y), (0, 0));
    }

    #[test]
    fn test_tiled_to_linear() {
        let tile = TileConfig::TILE_8X8;
        let index = tiled_to_linear(0, 0, tile, 64);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_tile_roundtrip() {
        let tile = TileConfig::TILE_16X16;
        let total_width = 128;
        
        for i in 0..256 {
            let (x, y) = linear_to_tiled(i, tile, total_width);
            let back = tiled_to_linear(x, y, tile, total_width);
            assert_eq!(i, back);
        }
    }

    #[test]
    fn test_tile_not_square() {
        let tile = TileConfig::new(16, 8);
        assert!(!tile.is_square());
        assert_eq!(tile.area(), 128);
    }
}
