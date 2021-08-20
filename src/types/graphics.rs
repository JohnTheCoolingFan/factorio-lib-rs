use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign};
use std::fmt;
use std::str::FromStr;
use crate::prototypes::PrototypesErr;
use crate::types::{Factorio2DVector, Color, FileName};

// ============ // Simple types // ============ //

/// List of 1-based frame indices into the spreadsheet
/// <https://wiki.factorio.com/Types/AnimationFrameSequence>
pub type AnimationFrameSequence = Vec<u16>;
/// <https://wiki.factorio.com/Types/Sprite#position>
pub type SpritePosition = (i16, i16);
/// Width and Height <https://wiki.factorio.com/Types/Sprite#width>
pub type SpriteSize = (i16, i16);

// =========== // General types // ============ //
        // Enums with FromStr

/// <https://wiki.factorio.com/Types/WorkingVisualisation#apply_recipe_tint>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ApplyRecipeTint {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    None,
}

impl FromStr for ApplyRecipeTint {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primary" => Ok(Self::Primary),
            "secondary" => Ok(Self::Secondary),
            "tertiary" => Ok(Self::Tertiary),
            "quaternary" => Ok(Self::Quaternary),
            "none" => Ok(Self::None),
            _ => Err(PrototypesErr::InvalidTypeStr("ApplyRecipeTint".into(), s.into()))
        }
    }
}

impl fmt::Display for ApplyRecipeTint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
            Self::Quaternary => "quaternary",
            Self::None => "none",
        })
    }
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation#apply_tint>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ApplyTint {
    ResourceColor,
    InputFluidBaseColor,
    InputFluidFlowColor,
    Status,
    None,
}

impl FromStr for ApplyTint {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "resource-color" => Ok(Self::ResourceColor),
            "input-fluid-base-color" => Ok(Self::InputFluidBaseColor),
            "input-fluid-flow-color" => Ok(Self::InputFluidFlowColor),
            "status" => Ok(Self::Status),
            "none" => Ok(Self::None),
            _ => Err(PrototypesErr::InvalidTypeStr("ApplyTint".into(), s.into()))
        }
    }
}

impl fmt::Display for ApplyTint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::ResourceColor => "resource-color",
            Self::InputFluidBaseColor => "input-fluid-base-color",
            Self::InputFluidFlowColor => "input-fluid-flow-color",
            Self::Status => "status",
            Self::None => "none",
        })
    }
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet#apply_module_tint>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ApplyModuleTint {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

impl FromStr for ApplyModuleTint {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primary" => Ok(Self::Primary),
            "secondary" => Ok(Self::Secondary),
            "tertiary" => Ok(Self::Tertiary),
            "quaternary" => Ok(Self::Quaternary),
            _ => Err(PrototypesErr::InvalidTypeStr("ApplyModuleTint".into(), s.into()))
        }
    }
}

impl fmt::Display for ApplyModuleTint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
            Self::Quaternary => "quaternary",
        })
    }
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet#module_tint_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ModuleTintMode {
    SingleModule,
    Mix,
}

impl FromStr for ModuleTintMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single-module" => Ok(Self::SingleModule),
            "mix" => Ok(Self::Mix),
            _ => Err(PrototypesErr::InvalidTypeStr("ModuleTintMode".into(), s.into()))
        }
    }
}

impl fmt::Display for ModuleTintMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::SingleModule => "single-module",
            Self::Mix => "mix",
        })
    }
}

/// <https://wiki.factorio.com/Types/LightDefinition#type>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LightDefinitionType {
    Basic,
    Oriented,
}

impl FromStr for LightDefinitionType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" => Ok(Self::Basic),
            "oriented" => Ok(Self::Oriented),
            _ => Err(PrototypesErr::InvalidTypeStr("LightDefinitionType".into(), s.into()))
        }
    }
}

impl fmt::Display for LightDefinitionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Basic => "basic",
            Self::Oriented => "oriented",
        })
    }
}

/// <https://wiki.factorio.com/Types/BaseAttackParameters#range_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RangeMode {
    CenterToCenter,
    BoundingBoxToBoundingBox,
}

impl FromStr for RangeMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center-to-center" => Ok(Self::CenterToCenter),
            "bounding-box-to-bounding-box" => Ok(Self::BoundingBoxToBoundingBox),
            _ => Err(PrototypesErr::InvalidTypeStr("RangeMode".into(), s.into()))
        }
    }
}

impl fmt::Display for RangeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::CenterToCenter => "center-to-center",
            Self::BoundingBoxToBoundingBox => "bounding-box-to-bounding-box",
        })
    }
}

/// <https://wiki.factorio.com/Prototype/Lamp#glow_render_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum GlowRenderMode {
    Additive,
    Multiplicative,
}

impl FromStr for GlowRenderMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "additive" => Ok(Self::Additive),
            "multiplicative" => Ok(Self::Multiplicative),
            _ => Err(PrototypesErr::InvalidTypeStr("GlowRenderMode".into(), s.into()))
        }
    }
}

impl fmt::Display for GlowRenderMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Additive => "additive",
            Self::Multiplicative => "multiplicative",
        })
    }
}

/// <https://wiki.factorio.com/Types/RenderLayer>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RenderLayer {
    WaterTile,
    GroundTile,
    TileTransition,
    Decals,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    Decorative,
    GroundPatch, // Love these names
    GroundPatchHigher,
    GroundPatchHigher2,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    LowerObject,
    TransportBeltCircuitConnector,
    LowerObjectAboveShadow,
    Object,
    HigherObjectUnder,
    HigherObjectAbove,
    ItemInInserterHand,
    Wires,
    WiresAbove,
    EntityInfoIcon,
    EntityInfoIconAbove,
    Explosion,
    Projectile,
    Smoke,
    AirObject,
    AirEntityInfoIcon,
    LightEffect,
    SelectionBox,
    HigherSelectionBox,
    CollisionSelectionBox,
    Arrow,
    Cursor
}

impl FromStr for RenderLayer {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "water-tile" => Ok(Self::WaterTile),
            "ground-tile" => Ok(Self::GroundTile),
            "tile-transition" => Ok(Self::TileTransition),
            "decals" => Ok(Self::Decals),
            "lower-radius-visualization" => Ok(Self::LowerRadiusVisualization),
            "radius-visualization" => Ok(Self::RadiusVisualization),
            "transport-belt-integration" => Ok(Self::TransportBeltIntegration),
            "resource" => Ok(Self::Resource),
            "building-smoke" => Ok(Self::BuildingSmoke),
            "decorative" => Ok(Self::Decorative),
            "ground-patch" => Ok(Self::GroundPatch),
            "ground-patch-higher" => Ok(Self::GroundPatchHigher),
            "ground-patch-higher2" => Ok(Self::GroundPatchHigher2),
            "remnants" => Ok(Self::Remnants),
            "floor" => Ok(Self::Floor),
            "transport-belt" => Ok(Self::TransportBelt),
            "transport-belt-endings" => Ok(Self::TransportBeltEndings),
            "floor-mechanics-under-corpse" => Ok(Self::FloorMechanicsUnderCorpse),
            "corpse" => Ok(Self::Corpse),
            "floor-mechanics" => Ok(Self::FloorMechanics),
            "item" => Ok(Self::Item),
            "lower-object" => Ok(Self::LowerObject),
            "transport-belt-circuit-connector" => Ok(Self::TransportBeltCircuitConnector),
            "lower-object-above-shadow" => Ok(Self::LowerObjectAboveShadow),
            "object" => Ok(Self::Object),
            "higher-object-under" => Ok(Self::HigherObjectUnder),
            "higher-object-above" => Ok(Self::HigherObjectAbove),
            "item-in-inserter-hand" => Ok(Self::ItemInInserterHand),
            "wires" => Ok(Self::Wires),
            "wires-above" => Ok(Self::WiresAbove),
            "entity-info-icon" => Ok(Self::EntityInfoIcon),
            "entity-info-icon-above" => Ok(Self::EntityInfoIconAbove),
            "explosion" => Ok(Self::Explosion),
            "projectile" => Ok(Self::Projectile),
            "smoke" => Ok(Self::Smoke),
            "air-object" => Ok(Self::AirObject),
            "air-entity-info-icon" => Ok(Self::AirEntityInfoIcon),
            "light-effect" => Ok(Self::LightEffect),
            "selection-box" => Ok(Self::SelectionBox),
            "higher-selection-box" => Ok(Self::HigherSelectionBox),
            "collision-selection-box" => Ok(Self::CollisionSelectionBox),
            "arrow" => Ok(Self::Arrow),
            "cursor" => Ok(Self::Cursor),
            _ => Err(PrototypesErr::InvalidTypeStr("RenderLayer".into(), s.into()))
        }
    }
}

impl fmt::Display for RenderLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::WaterTile => "water-tile",
            Self::GroundTile => "ground-tile",
            Self::TileTransition => "tile-transition",
            Self::Decals => "decals",
            Self::LowerRadiusVisualization => "lower-radius-visualization",
            Self::RadiusVisualization => "radius-visualization",
            Self::TransportBeltIntegration => "transport-belt-integration",
            Self::Resource => "resource",
            Self::BuildingSmoke => "building-smoke",
            Self::Decorative => "decorative",
            Self::GroundPatch => "ground-patch",
            Self::GroundPatchHigher => "ground-patch-higher",
            Self::GroundPatchHigher2 => "ground-patch-higher2",
            Self::Remnants => "remnants",
            Self::Floor => "floor",
            Self::TransportBelt => "transport-belt",
            Self::TransportBeltEndings => "transport-belt-endings",
            Self::FloorMechanicsUnderCorpse => "floor-mechanics-under-corpse",
            Self::Corpse => "corpse",
            Self::FloorMechanics => "floor-mechanics",
            Self::Item => "item",
            Self::LowerObject => "lower-object",
            Self::TransportBeltCircuitConnector => "transport-belt-circuit-connector",
            Self::LowerObjectAboveShadow => "lower-object-above-shadow",
            Self::Object => "object",
            Self::HigherObjectUnder => "higher-object-under",
            Self::HigherObjectAbove => "higher-object-above",
            Self::ItemInInserterHand => "item-in-inserter-hand",
            Self::Wires => "wires",
            Self::WiresAbove => "wires-above",
            Self::EntityInfoIcon => "entity-info-icon",
            Self::EntityInfoIconAbove => "entity-info-icon-above",
            Self::Explosion => "explosion",
            Self::Projectile => "projectile",
            Self::Smoke => "smoke",
            Self::AirObject => "air-object",
            Self::AirEntityInfoIcon => "air-entity-info-icon",
            Self::LightEffect => "light-effect",
            Self::SelectionBox => "selection-box",
            Self::HigherSelectionBox => "higher-selection-box",
            Self::CollisionSelectionBox => "collision-selection-box",
            Self::Arrow => "arrow",
            Self::Cursor => "cursor",
        })
    }
}

/// <https://wiki.factorio.com/Types/Sprite#draw_as_shadow>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DrawAs {
    DrawAsShadow,
    DrawAsGlow,
    DrawAsLight,
    None
}

impl DrawAs {
    pub fn new(draw_as_shadow: bool, draw_as_glow: bool, draw_as_light: bool) -> Self {
        if draw_as_shadow {
            Self::DrawAsShadow
        } else if draw_as_glow {
            Self::DrawAsGlow
        } else if draw_as_light {
            Self::DrawAsLight
        } else {
            Self::None
        }
    }
}

/// <https://wiki.factorio.com/Types/Sprite#blend_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BlendMode {
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    Overwrite
}

impl FromStr for BlendMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "additive" => Ok(Self::Additive),
            "additive-soft" => Ok(Self::AdditiveSoft),
            "multiplicative" => Ok(Self::Multiplicative),
            "overwrite" => Ok(Self::Overwrite),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("BlendMode"), String::from(s)))
        }
    }
}

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Normal => "normal",
            Self::Additive => "additive",
            Self::AdditiveSoft => "additive-soft",
            Self::Multiplicative => "multiplicative",
            Self::Overwrite => "overwrite",
        })
    }
}

/// <https://wiki.factorio.com/Types/Animation#run_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RunMode {
    Forward,
    Backward,
    ForwardThenBackward
}

impl FromStr for RunMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            "forward-then-backward" => Ok(Self::ForwardThenBackward),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("RunMode"), String::from(s)))
        }
    }
}

impl fmt::Display for RunMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Forward => "forward",
            Self::Backward => "backward",
            Self::ForwardThenBackward => "forward-then-backward",
        })
    }
}

        // Structs

/// <https://wiki.factorio.com/Types/FluidBox#secondary_draw_orders>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SecondaryDrawOrders {
    north: i8,
    east: i8,
    south: i8,
    west: i8
}

/// <https://wiki.factorio.com/Types/LightDefinition>
#[derive(Debug)]
pub struct LightDefinition {
    r#type: LightDefinitionType,
    // Next 2 are not optional if type is "oriented"
    picture: Option<Sprite>,
    rotation_shift: Option<f32>,
    intensity: f32,
    size: f32,
    source_orientation_offset: f32, // Default: 0
    add_perspective: bool, // Default: false
    shift: Option<Factorio2DVector>,
    color: Color, // Default: no color
    minimum_darkness: f32 // Default: 0
}

/// <https://wiki.factorio.com/Types/LightFlickeringDefinition>
#[derive(Debug)]
pub struct LightFlickeringDefinition {
    minimum_intensity: f32, // Default: 0.2
    maximum_intensity: f32, // Default: 0.8
    derivation_change_frequency: f32, // Default: 0.3
    derivation_change_deviation: f32, // Default: 0.06
    border_fix_speed: f32, // Default: 0.02
    minimum_light_size: f32, // Default: 0.5
    light_intensity_to_size_coefficient: f32, // Default: 0.5
    color: Color // Default: (1, 1, 1) (White)
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#default_recipe_tint>
#[derive(Debug)]
pub struct RecipeTint {
    // All default to (1, 1, 1, 1)
    primary: Color,
    secondary: Color,
    tertiary: Color,
    quaternary: Color
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#shift_animation_waypoints>
#[derive(Debug)]
pub struct ShiftAnimationWaypoints {
    north: Option<Vec<Factorio2DVector>>,
    east: Option<Vec<Factorio2DVector>>,
    south: Option<Vec<Factorio2DVector>>,
    west: Option<Vec<Factorio2DVector>>
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#status_colors>
#[derive(Debug)]
pub struct StatusColors {
    idle: Color, // Default: White (1, 1, 1)
    no_minable_resources: Color, // Default: `idle`
    full_output: Color, // Default: `idle`
    insufficient_output: Color, // Default: `idle`
    disabled: Color, // Default: `idle`
    no_power: Color, // Default: No color
    working: Color, // Default: White (1, 1, 1)
    low_power: Color, // Default: `working`
}

/// <https://wiki.factorio.com/Types/MiningDrillGraphicsSet#circuit_connector_secondary_draw_order>
#[derive(Debug)]
pub struct CircuitConnectorSecondaryDrawOrder {
    north: i8,
    east: i8,
    south: i8,
    west: i8
}

impl CircuitConnectorSecondaryDrawOrder {
    pub fn new(draw_order: i8) -> Self {
        Self{north: draw_order, east: draw_order, south: draw_order, west: draw_order}
    }
}

/// <https://wiki.factorio.com/Prototype/Entity#radius_visualisation_specification>
#[derive(Debug)]
pub struct RadiusVisualizationSpecification {
    sprite: Option<Sprite>,
    distance: f64, // Default: 0 // Must be > 0
    offset: Option<Factorio2DVector>,
    draw_in_cursor: bool, // Default: true
    draw_on_selection: bool // Default: true
}

/// <https://wiki.factorio.com/Types/WaterReflectionDefinition>
#[derive(Debug)]
pub struct WaterReflectionDefinition {
    pictures: Option<Vec<SpriteVariation>>,
    orientation_to_variation: bool, // default: false
    rotate: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/Sprite#slice_or_dice>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Dice(i16, i16);

impl Dice {
    pub fn new(n: i16) -> Self {
        Self(n, n)
    }

    pub fn new_xy(x: i16, y: i16) -> Self {
        Self(x, y)
    }
}

// ============= // Animations // ============= //

/// <https://wiki.factorio.com/Types/Animation#layers>
#[derive(Debug)]
pub enum Animation {
    Layers(Vec<Animation>),
    Single(AnimationBase)
}

/// <https://wiki.factorio.com/Types/Animation#hr_version>
#[derive(Debug)]
pub struct AnimationBase {
    regular: AnimationVariant,
    hr_version: Option<AnimationVariant>,
}

/// <https://wiki.factorio.com/Types/Animation#stripes>
#[derive(Debug)]
pub enum AnimationVariant {
    Regular(AnimationSpec),
    Stripes(Vec<Stripe>)
}

/// <https://wiki.factorio.com/Types/Animation>
#[derive(Debug)]
pub struct AnimationSpec {
    // These types share same fields/values, so I decided to "combine" them
    sprite: SpriteSpec,
    run_mode: RunMode, // Default: "forward"
    frame_count: u32, // Default: 1, can't be 0
    line_length: u32, // Default: 0
    animation_speed: f32, // Default: 1.0
    max_advance: f32, // Default: MAX_FLOAT
    repeat_count: u8, // Default: 1, can't be 0
    frame_sequence: Option<AnimationFrameSequence>,
}

/// <https://wiki.factorio.com/Types/Stripe>
#[derive(Debug)]
pub struct Stripe {
    width_in_frames: u32,
    height_in_frames: u32,
    filename: FileName,
    x: Option<u32>,
    y: Option<u32>
}

/// <https://wiki.factorio.com/Types/AnimationVariations>
pub type AnimationVariations = Vec<AnimationVariation>;

/// <https://wiki.factorio.com/Types/AnimationVariations>
#[derive(Debug)]
pub struct AnimationVariation {
    animation: AnimationBase, // Filename is mandatory
    variation_count: u32,
    frame_count: u32, // Default: 1
    line_length: u32, // Default: variation_count
}

/// <https://wiki.factorio.com/Types/Animation4Way>
#[derive(Debug)]
pub struct Animation4Way {
    // All fancy shenanigans are omitted, this program/library behaves like a game
    north: Animation,
    east: Animation,
    south: Animation,
    west: Animation,
}

/// <https://wiki.factorio.com/Types/AnimationElement>
#[derive(Debug)]
pub struct AnimationElement {
    render_layer: RenderLayer, // Default: "object"
    secondary_draw_order: Option<i8>,
    draw_as_sprite: bool, // Default: true
    draw_as_light: bool, // Default: false
    apply_tint: bool, // Default: false
    always_draw: bool, // Default: true
    animation: Animation
}

/// <https://wiki.factorio.com/Types/RotatedAnimation>
#[derive(Debug)]
pub struct RotatedAnimation {
    regular: RotatedAnimationSpec,
    hr_version: Option<RotatedAnimationSpec>
}

// A: "Are you sure this will work?"; Me: "I have no idea!"
/// <https://wiki.factorio.com/Types/RotatedAnimation>
#[derive(Debug)]
pub struct RotatedAnimationSpec {
    direction_count: u32,
    still_frame: u32, // Default: 0
    axially_symmetrical: bool, // Default: false
    counterclockwise: bool, // Default: false
    middle_orientation: f32, // Default: 0.5
    orientation_range: f32, // Default: 1
    apply_projection: bool, // Default: true
    animation: AnimationVariant
}

/// <https://wiki.factorio.com/Types/RotatedAnimationVariations>
pub type RotatedAnimationVariations = Vec<RotatedAnimationVariation>;

/// <https://wiki.factorio.com/Types/RotatedAnimationVariations>
#[derive(Debug)]
pub enum RotatedAnimationVariation {
    Layers(Vec<RotatedAnimationVariation>),
    Single(RotatedAnimation)
}

/// <https://wiki.factorio.com/Prototype/Beam#light_animations>
#[derive(Debug)]
pub struct LightAnimations {
    start: Option<Animation>,
    ending: Option<Animation>,
    head: Option<Animation>,
    tail: Option<Animation>,
    body: Option<Vec<AnimationVariation>>
}

// ============== // Sprites // ==============  //

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug)]
pub struct Sprite {
    layers: Vec<SpriteLayer>
}

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug)]
pub struct SpriteLayer {
    regular: SpriteSpec,
    hr_version: Option<SpriteSpec>
}

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug)]
pub struct SpriteSpec {
    filename: FileName,
    dice: Option<Dice>, // AKA slice // _y and _x are converted into this
    priority: SpritePriority, // Default: "medium"
    flags: Option<SpriteFlags>,
    size: Option<SpriteSize>,
    // Automatically converted to size
    // width
    // height
    position: Option<SpritePosition>,
    // Automatically converted to position
    // x
    // y
    shift: Factorio2DVector, // (0, 0) by default
    scale: f64, // 1 by default,
    draw_as: DrawAs, // all false by default
    mipmap_count: u8, // Default: 0
    apply_runtime_tint: bool, // Default: false
    tint: Color, // Default: (1, 1, 1, 1) (white)
    blend_mode: BlendMode, // Default: "normal"
    load_in_minimal_mode: bool, //Default: false
    premul_alpha: bool, // Default: true
    generate_sfd: bool // Default: false // Unused (Then why it is documented?)
}

/// <https://wiki.factorio.com/Types/SpriteNWaySheet>
#[derive(Debug)]
pub struct SpriteNWaySheet {
    sprite: SpriteSpec,
    frames: u32, // 4 or 8
}

/// <https://wiki.factorio.com/Types/Sprite4Way>
#[derive(Debug)]
pub struct Sprite4Way {
    // Priority list (most priority first): sheets, sheet, other properties
    sheets: Vec<SpriteNWaySheet>
}

/// <https://wiki.factorio.com/Types/Sprite8Way>
#[derive(Debug)]
pub struct Sprite8Way {
    // Priority list (most priority first): sheets, sheet, other properties
    sheets: Vec<SpriteNWaySheet>
}

/// <https://wiki.factorio.com/Types/RotatedSprite#layers>
#[derive(Debug)]
pub struct RotatedSprite {
    layers: Vec<RotatedSpriteLayer>
}

/// <https://wiki.factorio.com/Types/RotatedSprite>
#[derive(Debug)]
pub struct RotatedSpriteLayer {
    regular: RotatedSpriteSpec,
    hr_version: Option<RotatedSpriteSpec>
}

/// <https://wiki.factorio.com/Types/RotatedSprite>
#[derive(Debug)]
pub struct RotatedSpriteSpec {
    sprites: Vec<SpriteSpec>, // If `filenames` is set, copy all properties to each object for each filename
    direction_count: u16
}

/// <https://wiki.factorio.com/Types/SpriteVariations>
pub type SpriteVariations = Vec<SpriteVariation>;

/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug)]
pub struct SpriteVariation {
    layers: Vec<SpriteVariationLayer>
}

/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug)]
pub struct SpriteVariationLayer {
    regular: SpriteVariationSpec,
    hr_version: Option<SpriteVariationSpec>
}

/// Extension of SpriteSpec, ignores dice and slice
/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug)]
pub struct SpriteVariationSpec {
    sprite: SpriteSpec,
    variation_count: u32, // Default: 1
    repeat_count: u32, // Default: 1
    line_length: u32 // Default: value of `variation_count`
}

/// <https://wiki.factorio.com/Types/SpriteFlags>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SpriteFlags(u32);

impl SpriteFlags {
    pub const NO_CROP: SpriteFlags = SpriteFlags(1);
    pub const NOT_COMPRESSED: SpriteFlags = SpriteFlags(1 << 1);
    pub const ALWAYS_COMPRESSED: SpriteFlags = SpriteFlags(1 << 2);
    pub const MIPMAP: SpriteFlags = SpriteFlags(1 << 3);
    pub const LINEAR_MINIFICATION: SpriteFlags = SpriteFlags(1 << 4);
    pub const LINEAR_MAGNIFICATION: SpriteFlags = SpriteFlags(1 << 5);
    pub const LINEAR_MIP_LEVEL: SpriteFlags = SpriteFlags(1 << 6);
    pub const ALPHA_MASK: SpriteFlags = SpriteFlags(1 << 7);
    pub const NO_SCALE: SpriteFlags = SpriteFlags(1 << 8);
    pub const MASK: SpriteFlags = SpriteFlags(1 << 9);
    pub const ICON: SpriteFlags = SpriteFlags(1 << 10);
    pub const GUI: SpriteFlags = SpriteFlags(1 << 11);
    pub const GUI_ICON: SpriteFlags = SpriteFlags(1 << 12);
    pub const LIGHT: SpriteFlags = SpriteFlags(1 << 13);
    pub const TERRAIN: SpriteFlags = SpriteFlags(1 << 14);
    pub const TERRAIN_EFFECT_MAP: SpriteFlags = SpriteFlags(1 << 15);
    pub const SHADOW: SpriteFlags = SpriteFlags(1 << 16);
    pub const SMOKE: SpriteFlags = SpriteFlags(1 << 17);
    pub const DECAL: SpriteFlags = SpriteFlags(1 << 18);
    pub const LOW_OBJECT: SpriteFlags = SpriteFlags(1 << 19);
    pub const TRILINEAR_FILTERING: SpriteFlags = SpriteFlags(1 << 20);
    pub const GROUP_NONE: SpriteFlags = SpriteFlags(1 << 21);
    pub const GROUP_TERRAIN: SpriteFlags = SpriteFlags(1 << 22);
    pub const GROUP_TERRAIN_EFFECT_MAP: SpriteFlags = SpriteFlags(1 << 23);
    pub const GROUP_SHADOW: SpriteFlags = SpriteFlags(1 << 24);
    pub const GROUP_SMOKE: SpriteFlags = SpriteFlags(1 << 25);
    pub const GROUP_DECAL: SpriteFlags = SpriteFlags(1 << 26);
    pub const GROUP_LOW_OBJECT: SpriteFlags = SpriteFlags(1 << 27);
    pub const GROUP_GUI: SpriteFlags = SpriteFlags(1 << 28);
    pub const GROUP_ICON: SpriteFlags = SpriteFlags(1 << 29);
    pub const GROUP_ICON_BACKGROUND: SpriteFlags = SpriteFlags(1 << 30);
    pub const COMPRESSED: SpriteFlags = SpriteFlags(1 << 31);
}

impl From<Vec<&str>> for SpriteFlags {
    fn from(flags: Vec<&str>) -> Self {
        let mut result = Self(0);
        for flag in flags {
            match flag {
                "no-crop" => result |= SpriteFlags::NO_CROP,
                "not-compressed" => result |= SpriteFlags::NOT_COMPRESSED,
                "always-compressed" => result |= SpriteFlags::ALWAYS_COMPRESSED,
                "mipmap" => result |= SpriteFlags::MIPMAP,
                "linear-minification" => result |= SpriteFlags::LINEAR_MINIFICATION,
                "linear-maginfication" => result |= SpriteFlags::LINEAR_MAGNIFICATION,
                "linear-mip-level" => result |= SpriteFlags::LINEAR_MIP_LEVEL,
                "alpha-mask" => result |= SpriteFlags::ALPHA_MASK,
                "no-scale" => result |= SpriteFlags::NO_SCALE,
                "mask" => result |= SpriteFlags::MASK | SpriteFlags::GROUP_NONE,
                "icon" => result |= SpriteFlags::ICON |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::NO_SCALE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::NOT_COMPRESSED |
                    SpriteFlags::GROUP_ICON,
                "gui" => result |= SpriteFlags::GUI |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::NO_SCALE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::NOT_COMPRESSED |
                    SpriteFlags::GROUP_GUI,
                "gui-icon" => result |= SpriteFlags::GUI_ICON |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::NO_SCALE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::NOT_COMPRESSED |
                    SpriteFlags::GROUP_ICON,
                "light" => result |= SpriteFlags::LIGHT |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::GROUP_NONE,
                "terrain" => result |= SpriteFlags::TERRAIN |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::GROUP_TERRAIN,
                "terrain-effect-map" => result |= SpriteFlags::TERRAIN_EFFECT_MAP |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::GROUP_TERRAIN_EFFECT_MAP,
                "shadow" => result |= SpriteFlags::SHADOW,
                "smoke" => result |= SpriteFlags::SMOKE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::GROUP_SMOKE,
                "decal" => result |= SpriteFlags::DECAL |
                    SpriteFlags::GROUP_DECAL,
                "low-object" => result |= SpriteFlags::LOW_OBJECT,
                "trilinear-filtering" => result |= SpriteFlags::TRILINEAR_FILTERING,
                /*
                "group=none" => result |= SpriteFlags::GROUP_NONE,
                "group=terrain" => result |= SpriteFlags::GROUP_TERRAIN,
                "group=terrain-effect-map" => result |= SpriteFlags::GROUP_TERRAIN_EFFECT_MAP,
                "group=shadow" => result |= SpriteFlags::GROUP_SHADOW,
                "group=smoke" => result |= SpriteFlags::GROUP_SMOKE,
                "group=decal" => result |= SpriteFlags::GROUP_DECAL,
                "group=low-object" => result |= SpriteFlags::GROUP_LOW_OBJECT,
                "group=gui" => result |= SpriteFlags::GROUP_GUI,
                "group=icon" => result |= SpriteFlags::GROUP_ICON,
                "group=icon-background" => result |= SpriteFlags::GROUP_ICON_BACKGROUND, */
                "compressed" => result |= SpriteFlags::COMPRESSED,
                _ => {}
            }
        }
        result
    }
}

impl BitAnd for SpriteFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        SpriteFlags(self.0 & rhs.0)
    }
}

impl BitAndAssign for SpriteFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = SpriteFlags(self.0 & rhs.0)
    }
}

impl BitOr for SpriteFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        SpriteFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for SpriteFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = SpriteFlags(self.0 | rhs.0)
    }
}

impl BitXor for SpriteFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        SpriteFlags(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for SpriteFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = SpriteFlags(self.0 ^ rhs.0)
    }
}

/// <https://wiki.factorio.com/Types/Sprite#priority>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SpritePriority {
    ExtraHighNoScale,
    ExtraHigh,
    High,
    Medium,
    Low,
    VeryLow,
    NoAtlas
}

impl fmt::Display for SpritePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::ExtraHighNoScale => "extra-high-no-scale",
            Self::ExtraHigh => "extra-high",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::VeryLow => "very-low",
            Self::NoAtlas => "no-atlas",
        })
    }
}

impl FromStr for SpritePriority {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "extra-high-no-scale" => Ok(Self::ExtraHighNoScale),
            "extra-high" => Ok(Self::ExtraHigh),
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            "very-low" => Ok(Self::VeryLow),
            "no-atlas" => Ok(Self::NoAtlas),
            _ => Err(PrototypesErr::InvalidTypeStr("SpritePriority".into(), s.into()))
        }
    }
}

// ===== // Graphics Sets and Pictures // ===== //

/// <https://wiki.factorio.com/Prototype/Rail#pictures>
#[derive(Debug)]
pub struct RailPictures {
    straight_rail_horizontal: RailPieceLayers,
    straight_rail_vertical: RailPieceLayers,
    straight_rail_diagonal_left_top: RailPieceLayers,
    straight_rail_diagonal_right_top: RailPieceLayers,
    straight_rail_diagonal_right_bottom: RailPieceLayers,
    straight_rail_diagonal_left_bottom: RailPieceLayers,
    curved_rail_vertical_left_top: RailPieceLayers,
    curved_rail_vertical_right_top: RailPieceLayers,
    curved_rail_vertical_right_bottom: RailPieceLayers,
    curved_rail_vertical_left_bottom: RailPieceLayers,
    curved_rail_horizontal_left_top: RailPieceLayers,
    curved_rail_horizontal_right_top: RailPieceLayers,
    curved_rail_horizontal_right_bottom: RailPieceLayers,
    curved_rail_horizontal_left_bottom: RailPieceLayers,
    rail_endings: Sprite8Way
}

/// <https://wiki.factorio.com/Types/RailPieceLayers>
#[derive(Debug)]
pub struct RailPieceLayers {
    metals: Vec<SpriteVariation>,
    backplayes: Vec<SpriteVariation>, // Must have same number of variations as `metals`
    ties: Vec<SpriteVariation>, // Must have between 1 and 4 items
    stone_path: Vec<SpriteVariation>, // Must have between 1 and 4 items
    stone_path_background: Option<Vec<SpriteVariation>>,
    segment_visualisation_middle: Option<Sprite>,
    segment_visualisation_ending_front: Option<Sprite>,
    segment_visualisation_ending_back: Option<Sprite>,
    segment_visualisation_continuing_front: Option<Sprite>,
    segment_visualisation_continuing_back: Option<Sprite>,
}

/// <https://wiki.factorio.com/Types/CircuitConnectorSprites>
#[derive(Debug)]
pub struct CircuitConnectorSprites {
    led_red: Sprite,
    led_green: Sprite,
    led_blue: Sprite,
    led_light: LightDefinition,
    connector_main: Option<Sprite>,
    connector_shadow: Option<Sprite>,
    wire_pins: Option<Sprite>,
    wire_pins_shadow: Option<Sprite>,
    led_blue_off: Option<Sprite>,
    blue_led_light_offset: Option<Factorio2DVector>,
    red_green_led_light_offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet>
#[derive(Debug)]
pub struct BeaconGraphicsSet {
    draw_animation_when_idle: bool, //Default: true
    draw_light_when_idle: bool, // Default: false
    random_animation_offset: bool, // Default: false
    module_icons_suppressed: bool, // Default: false
    base_layer: RenderLayer, // Default: "object"
    animation_layer: RenderLayer, // Default: "object"
    top_layer: RenderLayer, // Default: "object"
    animation_progress: f32, // Default: 1
    min_animation_progress: f32, // Default: 0
    max_animation_progress: f32, // Default: 1000
    apply_module_tint: Option<ApplyModuleTint>, // Default: "none"
    apply_module_tint_to_light: Option<ApplyModuleTint>, // Default: "none"
    no_modules_tint: Color, //Default: no color
    animation_list: Option<Vec<AnimationElement>>,
    light: Option<LightDefinition>,
    module_visualisations: Option<BeaconModuleVisualizations>,
    module_tint_mode: ModuleTintMode // Default: "single-module"
}

/// <https://wiki.factorio.com/Types/BeaconModuleVisualizations>
#[derive(Debug)]
pub struct BeaconModuleVisualizations {
    art_style: String,
    use_for_empty_slots: bool, // Default: false
    tier_offset: i32, // Default: 0
    slots: Option<Vec<Vec<BeaconModuleVisualization>>>
}

/// <https://wiki.factorio.com/Types/BeaconModuleVisualization>
#[derive(Debug)]
pub struct BeaconModuleVisualization {
    has_empty_slot: bool, // Default: false
    draw_as_light: bool, // Default: false
    draw_as_sprite: bool, // Default: true
    secondary_draw_order: i8, // Default: 0
    apply_module_tint: ApplyModuleTint, // Default: "none"
    render_layer: RenderLayer, // Default: "object"
    pictures: Option<Vec<SpriteVariation>>
}

/// <https://wiki.factorio.com/Types/CharacterArmorAnimation>
#[derive(Debug)]
pub struct CharacterArmorAnimation {
    idle: RotatedAnimation,
    idle_with_gun: RotatedAnimation,
    running: RotatedAnimation,
    running_with_gun: RotatedAnimation,
    mining_with_tool: RotatedAnimation,
    flipped_shadow_running_with_gun: Option<RotatedAnimation>, // If set, must containt exactly 18 directions
    armors: Option<Vec<String>>,
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation>
#[derive(Debug)]
pub struct WorkingVisualisation {
    render_layer: RenderLayer, // Default: "object"
    fadeout: bool, // Default: false
    synced_fadeout: bool, // Default: false
    constant_speed: bool, // Default: false
    always_draw: bool, // Default: false
    animated_shift: bool, // Default: false
    align_to_waypoint: bool, // Default: false
    secondary_draw_order: Option<i8>,
    draw_as_sprite: bool, // Default: true
    draw_as_light: bool, // Default: false
    light: Option<LightDefinition>,
    effect: Option<WorkingVisualisationEffect>,
    apply_recipe_tint: Option<ApplyRecipeTint>,
    apply_tint: Option<ApplyTint>,
    north_animation: Option<Animation>,
    west_animation: Option<Animation>,
    south_animation: Option<Animation>,
    east_animation: Option<Animation>,
    animation: Option<Animation>,
    north_position: Option<Factorio2DVector>,
    west_position: Option<Factorio2DVector>,
    south_position: Option<Factorio2DVector>,
    east_position: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation#effect>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum WorkingVisualisationEffect {
    Flicker,
    UraniumGlow,
    None,
}

impl FromStr for WorkingVisualisationEffect {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "flicker" => Ok(Self::Flicker),
            "uranium-glow" => Ok(Self::UraniumGlow),
            "none" => Ok(Self::None),
            _ => Err(PrototypesErr::InvalidTypeStr("WorkingVisualisationEffect".into(), s.into()))
        }
    }
}

impl fmt::Display for WorkingVisualisationEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Flicker => "flicker",
            Self::UraniumGlow => "uranium-glow",
            Self::None => "none",
        })
    }
}

/// <https://wiki.factorio.com/Types/ConnectableEntityGraphics>
#[derive(Debug)]
pub struct ConnectableEntityGraphics {
    single: Vec<SpriteVariation>,
    straight_vertical: Vec<SpriteVariation>,
    straight_horizontal: Vec<SpriteVariation>,
    corner_right_down: Vec<SpriteVariation>,
    corner_left_down: Vec<SpriteVariation>,
    corner_right_up: Vec<SpriteVariation>,
    corner_left_up: Vec<SpriteVariation>,
    t_up: Vec<SpriteVariation>,
    t_right: Vec<SpriteVariation>,
    t_down: Vec<SpriteVariation>,
    t_left: Vec<SpriteVariation>,
    ending_up: Vec<SpriteVariation>,
    ending_right: Vec<SpriteVariation>,
    ending_down: Vec<SpriteVariation>,
    ending_left: Vec<SpriteVariation>,
    cross: Vec<SpriteVariation>,
}

/// <https://wiki.factorio.com/Types/MiningDrillGraphicsSet>
#[derive(Debug)]
pub struct MiningDrillGraphicsSet {
    animation: Option<Animation4Way>,
    idle_animation: Option<Animation4Way>,
    always_draw_idle_animation: bool, // Default: false
    default_recipe_tint: Option<RecipeTint>,
    working_visualisations: Option<WorkingVisualisation>,
    /// Only loaded if `shift_animation_waypoint_stop_duration` or `shift_animation_transition_duration` is not 0
    shift_animation_waypoints: Option<ShiftAnimationWaypoints>,
    shift_animation_waypoint_stop_duration: u16, // Default: 0
    shift_animation_transition_duration: u16, // Default: 0
    status_colors: Option<StatusColors>,
    drilling_vertical_movement_duration: u16, // Default: 0
    animation_progress: f32, // Default: 1
    max_animation_progress: f32, // Default: 1000
    min_animation_progress: f32, // Default: 0
    circuit_connector_layer: CircuitConnectorRenderLayers, // Default: all "object"
    circuit_connector_secondary_draw_order: CircuitConnectorSecondaryDrawOrder // Default: all 100
}

/// <https://wiki.factorio.com/Types/MiningDrillGraphicsSet#circuit_connector_layer>
#[derive(Debug)]
pub struct CircuitConnectorRenderLayers {
    north: RenderLayer,
    east: RenderLayer,
    south: RenderLayer,
    west: RenderLayer
}

impl CircuitConnectorRenderLayers {
    pub fn new(render_layer: RenderLayer) -> Self {
        Self{north: render_layer, east: render_layer, south: render_layer, west: render_layer}
    }
}

/// <https://wiki.factorio.com/Prototype/OffshorePump#graphics_set>
#[derive(Debug)]
pub struct OffshorePumpGraphicsSet {
    animation: Animation4Way,
    base_render_layer: RenderLayer, // Default: "ground-patch"
    underwater_layer_offset: i8, // Default: 1
    fluid_animation: Option<Animation4Way>,
    glass_pictures: Option<Sprite4Way>,
    base_pictures: Option<Sprite4Way>,
    underwater_pictures: Option<Sprite4Way>
}

/// <https://wiki.factorio.com/Prototype/Pipe#pictures>
#[derive(Debug)]
pub struct PipePictures {
    straight_vertical_single: Sprite,
    straight_vertical: Sprite,
    straight_vertical_window: Sprite,
    straight_horizontal: Sprite,
    straight_horizontal_window: Sprite,
    corner_up_right: Sprite,
    corner_up_left: Sprite,
    corner_down_right: Sprite,
    corner_down_left: Sprite,
    t_up: Sprite,
    t_down: Sprite,
    t_right: Sprite,
    t_left: Sprite,
    cross: Sprite,
    ending_up: Sprite,
    ending_down: Sprite,
    ending_right: Sprite,
    ending_left: Sprite,
    horizontal_window_background: Sprite,
    vertical_window_background: Sprite,
    fluid_background: Sprite,
    low_temperature_flow: Sprite,
    middle_temperature_flow: Sprite,
    high_temperature_flow: Sprite,
    gas_flow: Animation

}

/// <https://wiki.factorio.com/Prototype/PipeToGround#pictures>
#[derive(Debug)]
pub struct PipeToGroundPictures {
    down: Sprite,
    up: Sprite,
    left: Sprite,
    ritgh: Sprite
}

/// <https://wiki.factorio.com/Prototype/Pump#fluid_wagon_connector_graphics>
#[derive(Debug)]
pub struct PumpConnectorGraphicsFluidWagon {
    load_animations: PumpConnectorGraphics,
    unload_animations: PumpConnectorGraphics
}

/// <https://wiki.factorio.com/Types/PumpConnectorGraphics>
#[derive(Debug)]
pub struct PumpConnectorGraphics {
    north: Vec<PumpConnectorGraphicsMapping>,
    east: Vec<PumpConnectorGraphicsMapping>,
    south: Vec<PumpConnectorGraphicsMapping>,
    west: Vec<PumpConnectorGraphicsMapping>,
}

/// <https://wiki.factorio.com/Types/PumpConnectorGraphics>
#[derive(Debug)]
pub struct PumpConnectorGraphicsMapping {
    standup_base: Option<Animation>,
    standup_top: Option<Animation>,
    standup_shadow: Option<Animation>,
    connector: Option<Animation>,
    connector_shadow: Option<Animation>,
}
