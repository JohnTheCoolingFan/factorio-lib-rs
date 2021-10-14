use std::fmt;
use std::str::FromStr;
use crate::{types::{Sprite, Animation, FileName, SpriteSizeType}, prototypes::PrototypesErr};

/// <https://wiki.factorio.com/Types/TileTransitions>
#[derive(Debug)]
pub struct TileTransitionsBase {
    // 3 properties need to be specified if `empty_transitions` is false
    // They also correspond to *_mask names
    side: Option<TileTransitionSprite>,
    inner_corner: Option<TileTransitionSprite>,
    outer_corner: Option<TileTransitionSprite>,
    empty_transitions: bool, // default: false
    side_background: Option<TileTransitionSprite>, // And _mask
    side_effect_map: Option<TileTransitionSprite>,
    side_weights: Option<Vec<f32>>,
    inner_corner_background: Option<TileTransitionSprite>, // And _mask
    inner_corner_effect_map: Option<TileTransitionSprite>,
    inner_corner_weights: Option<Vec<f32>>,
    outer_corner_background: Option<TileTransitionSprite>, // namd _mask
    outer_corner_effect_map: Option<TileTransitionSprite>,
    outer_corner_weights: Option<Vec<f32>>,
    u_transition: Option<TileTransitionSprite>, // And _mask
    u_transition_background: Option<TileTransitionSprite>, // And _mask
    u_transition_effect_map: Option<TileTransitionSprite>,
    u_transition_weights: Option<Vec<f32>>,
    o_transition: Option<TileTransitionSprite>, // And _mask
    o_transition_background: Option<TileTransitionSprite>, // And _mask
    o_transition_effect_map: Option<TileTransitionSprite>,
    water_patch: Option<Sprite>,
    effect_mask: Option<Animation>,
    layer: u8,
    overlay_layer_group: Option<LayerGroup>,
    background_layer_group: Option<LayerGroup>,
    overlay_layer_offset: Option<i8>,
    masked_overlay_layer_offset: i8, // Default: 0
    background_layer_offset: i8, // Default: 0
    masked_background_layer_offset: Option<i8>,
    apply_effect_color_to_overlay: bool, // Default: false
    offset_background_layer_by_tile_layer: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/Tile#layer_group>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum LayerGroup {
    Zero,
    Water,
    WaterOverlay,
    Ground,
    Top,
}

impl FromStr for LayerGroup {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zero" => Ok(Self::Zero),
            "water" => Ok(Self::Water),
            "water-overlay" => Ok(Self::WaterOverlay),
            "ground" => Ok(Self::Ground),
            "top" => Ok(Self::Top),
            _ => Err(PrototypesErr::InvalidTypeStr("LayerGroup".into(), s.into()))
        }
    }
}

impl fmt::Display for LayerGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Zero => "zero",
            Self::Water => "water",
            Self::WaterOverlay => "water-overlay",
            Self::Ground => "ground",
            Self::Top => "top",
        })
    }
}

/// <https://wiki.factorio.com/Types/TileTransitionSprite>
#[derive(Debug)]
pub struct TileTransitionSprite {
    regular: TileTransitionSpriteSpec,
    hr_version: Option<TileTransitionSpriteSpec>
}

/// <https://wiki.factorio.com/Types/TileTransitionSprite>
#[derive(Debug)]
pub struct TileTransitionSpriteSpec {
    count: u32,
    picture: FileName,
    tall: bool, // Default: false
    scale: f32, // Default: 1
    x: SpriteSizeType, // Default: 0
    y: SpriteSizeType, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/Tile#variants>
#[derive(Debug)]
pub struct MainTileTransitions {
    base: TileTransitionsBase,
    main: Vec<MainTileSprite>,
    material_background: Option<TileSprite>
}

/// <https://wiki.factorio.com/Types/TileSprite>
#[derive(Debug)]
pub struct TileSprite {
    regular: TileSpriteSpec,
    hr_version: Option<TileSpriteSpec>
}

/// <https://wiki.factorio.com/Types/TileSprite>
#[derive(Debug)]
pub struct TileSpriteSpec {
    count: u32,
    picture: FileName,
    scale: f32, // Default: 1
    x: SpriteSizeType, // Default: 0
    y: SpriteSizeType, // Default: 0
    line_length: Option<u32>
}

/// <https://wiki.factorio.com/Prototype/Tile#variants>
#[derive(Debug)]
pub struct MainTileSprite {
    regular: MainTileSpriteSpec,
    hr_version: Option<MainTileSpriteSpec>
}

/// <https://wiki.factorio.com/Prototype/Tile#variants>
#[derive(Debug)]
pub struct MainTileSpriteSpec {
    base: TileSpriteSpec,
    size: u32, // Only power of 2 from 1 to 128 are accepted // So whitelist: [1, 2, 4, 8, 16, 32, 64, 128]
    probability: f64, // Default: 1
    weights: Option<Vec<f64>>
}

/// <https://wiki.factorio.com/Prototype/Tile#transitions>
#[derive(Debug)]
pub struct ExtraTileTransitions {
    base: TileTransitionsBase,
    to_tiles: String, // Name of a Tile
    transition_group: u8
}

/// <https://wiki.factorio.com/Prototype/Tile#transitions_between_transitions>
#[derive(Debug)]
pub struct BetweenTileTransitions {
    base: TileTransitionsBase,
    transition_group1: u8,
    transition_group2: u8
}