use super::{Sprite, Animation, FileName, SpriteSizeType};
use strum_macros::{EnumString, AsRefStr};

/// <https://wiki.factorio.com/Types/TileTransitions>
#[derive(Debug, Clone)]
pub struct TileTransitionsBase {
    // 3 properties need to be specified if `empty_transitions` is false
    // They also correspond to *_mask names
    pub side: Option<TileTransitionSprite>,
    pub inner_corner: Option<TileTransitionSprite>,
    pub outer_corner: Option<TileTransitionSprite>,
    pub empty_transitions: bool, // default: false
    pub side_background: Option<TileTransitionSprite>, // And _mask
    pub side_effect_map: Option<TileTransitionSprite>,
    pub side_weights: Option<Vec<f32>>,
    pub inner_corner_background: Option<TileTransitionSprite>, // And _mask
    pub inner_corner_effect_map: Option<TileTransitionSprite>,
    pub inner_corner_weights: Option<Vec<f32>>,
    pub outer_corner_background: Option<TileTransitionSprite>, // And _mask
    pub outer_corner_effect_map: Option<TileTransitionSprite>,
    pub outer_corner_weights: Option<Vec<f32>>,
    pub u_transition: Option<TileTransitionSprite>, // And _mask
    pub u_transition_background: Option<TileTransitionSprite>, // And _mask
    pub u_transition_effect_map: Option<TileTransitionSprite>,
    pub u_transition_weights: Option<Vec<f32>>,
    pub o_transition: Option<TileTransitionSprite>, // And _mask
    pub o_transition_background: Option<TileTransitionSprite>, // And _mask
    pub o_transition_effect_map: Option<TileTransitionSprite>,
    pub water_patch: Option<Sprite>,
    pub effect_mask: Option<Animation>,
    pub layer: u8,
    pub overlay_layer_group: Option<LayerGroup>,
    pub background_layer_group: Option<LayerGroup>,
    pub overlay_layer_offset: Option<i8>,
    pub masked_overlay_layer_offset: i8, // Default: 0
    pub background_layer_offset: i8, // Default: 0
    pub masked_background_layer_offset: Option<i8>,
    pub apply_effect_color_to_overlay: bool, // Default: false
    pub offset_background_layer_by_tile_layer: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/Tile#layer_group>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum LayerGroup {
    Zero,
    Water,
    WaterOverlay,
    Ground,
    Top,
}

/// <https://wiki.factorio.com/Types/TileTransitionSprite>
#[derive(Debug, Clone)]
pub struct TileTransitionSprite {
    regular: TileTransitionSpriteSpec,
    hr_version: Option<TileTransitionSpriteSpec>
}

/// <https://wiki.factorio.com/Types/TileTransitionSprite>
#[derive(Debug, Clone)]
pub struct TileTransitionSpriteSpec {
    count: u32,
    picture: FileName,
    tall: bool, // Default: false
    scale: f32, // Default: 1
    x: SpriteSizeType, // Default: 0
    y: SpriteSizeType, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/Tile#variants>
#[derive(Debug, Clone)]
pub struct MainTileTransitions {
    base: TileTransitionsBase,
    main: Vec<MainTileSprite>,
    material_background: Option<TileSprite>
}

/// <https://wiki.factorio.com/Types/TileSprite>
#[derive(Debug, Clone)]
pub struct TileSprite {
    regular: TileSpriteSpec,
    hr_version: Option<TileSpriteSpec>
}

/// <https://wiki.factorio.com/Types/TileSprite>
#[derive(Debug, Clone)]
pub struct TileSpriteSpec {
    count: u32,
    picture: FileName,
    scale: f32, // Default: 1
    x: SpriteSizeType, // Default: 0
    y: SpriteSizeType, // Default: 0
    line_length: Option<u32>
}

/// <https://wiki.factorio.com/Prototype/Tile#variants>
#[derive(Debug, Clone)]
pub struct MainTileSprite {
    regular: MainTileSpriteSpec,
    hr_version: Option<MainTileSpriteSpec>
}

/// <https://wiki.factorio.com/Prototype/Tile#variants>
#[derive(Debug, Clone)]
pub struct MainTileSpriteSpec {
    base: TileSpriteSpec,
    size: u32, // Only power of 2 from 1 to 128 are accepted // So whitelist: [1, 2, 4, 8, 16, 32, 64, 128]
    probability: f64, // Default: 1
    weights: Option<Vec<f64>>
}

/// <https://wiki.factorio.com/Prototype/Tile#transitions>
#[derive(Debug, Clone)]
pub struct ExtraTileTransitions {
    base: TileTransitionsBase,
    to_tiles: String, // Name of a Tile
    transition_group: u8
}

/// <https://wiki.factorio.com/Prototype/Tile#transitions_between_transitions>
#[derive(Debug, Clone)]
pub struct BetweenTileTransitions {
    base: TileTransitionsBase,
    transition_group1: u8,
    transition_group2: u8
}
