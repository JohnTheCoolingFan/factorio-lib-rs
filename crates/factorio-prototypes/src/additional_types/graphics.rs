use super::{
    BoundingBox, Color, CreateParticleTriggerEffectItem, Factorio2DVector, FileName,
    RealOrientation,
};
use super::{DataTable, GetPrototype, PrototypeFromLua};
use crate::util::defaults::*;
use factorio_lib_rs_derive::prot_from_str;
use mlua::{prelude::*, Value};
use serde::Deserialize;
use std::iter::{FromIterator, Iterator};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use strum_macros::{AsRefStr, EnumString};

// ============ // Simple types // ============ //

/// List of 1-based frame indices into the spreadsheet
/// <https://wiki.factorio.com/Types/AnimationFrameSequence>
pub type AnimationFrameSequence = Vec<u16>;

/// <https://wiki.factorio.com/Types/Sprite#position>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct SpritePosition(pub i16, pub i16);

/// Width and Height <https://wiki.factorio.com/Types/Sprite#width>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct SpriteSize(pub i16, pub i16);

/// <https://wiki.factorio.com/Types/SpriteSizeType>
pub type SpriteSizeType = i16;

// =========== // General types // ============ //
// Enums with FromStr

/// <https://wiki.factorio.com/Types/WorkingVisualisation#apply_recipe_tint>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ApplyRecipeTint {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    None,
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation#apply_tint>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ApplyTint {
    ResourceColor,
    InputFluidBaseColor,
    InputFluidFlowColor,
    Status,
    None,
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet#apply_module_tint>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ApplyModuleTint {
    None,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet#module_tint_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ModuleTintMode {
    SingleModule,
    Mix,
}

/// <https://wiki.factorio.com/Types/LightDefinition#type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum LightDefinitionType {
    Basic,
    Oriented,
}

/// <https://wiki.factorio.com/Types/BaseAttackParameters#range_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RangeMode {
    CenterToCenter,
    BoundingBoxToBoundingBox,
}

/// <https://wiki.factorio.com/Prototype/Lamp#glow_render_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum GlowRenderMode {
    Additive,
    Multiplicative,
}

/// <https://wiki.factorio.com/Types/RenderLayer>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
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
    Cursor,
}

/// <https://wiki.factorio.com/Types/Sprite#draw_as_shadow>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Deserialize)]
#[serde(from = "DrawAsIntermediate")]
pub enum DrawAs {
    DrawAsShadow,
    DrawAsGlow,
    DrawAsLight,
    None,
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

#[derive(Deserialize)]
struct DrawAsIntermediate {
    draw_as_shadow: Option<bool>,
    draw_as_glow: Option<bool>,
    draw_as_light: Option<bool>,
}

impl From<DrawAsIntermediate> for DrawAs {
    fn from(value: DrawAsIntermediate) -> Self {
        let DrawAsIntermediate {
            draw_as_shadow,
            draw_as_glow,
            draw_as_light,
        } = value;

        Self::new(
            draw_as_shadow.unwrap_or(false),
            draw_as_glow.unwrap_or(false),
            draw_as_light.unwrap_or(false),
        )
    }
}

/// <https://wiki.factorio.com/Types/Sprite#blend_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum BlendMode {
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    Overwrite,
}

/// <https://wiki.factorio.com/Types/Animation#run_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RunMode {
    Forward,
    Backward,
    ForwardThenBackward,
}

// Structs

/// <https://wiki.factorio.com/Types/FluidBox#secondary_draw_orders>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Deserialize)]
pub struct SecondaryDrawOrders {
    #[serde(default = "default_i8::<1>")]
    pub north: i8,
    #[serde(default = "default_i8::<1>")]
    pub east: i8,
    #[serde(default = "default_i8::<1>")]
    pub south: i8,
    #[serde(default = "default_i8::<1>")]
    pub west: i8,
}

/// <https://wiki.factorio.com/Types/LightDefinition>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LightDefinition {
    Single(LightDefinitionProperties),
    Multiple(Vec<LightDefinitionProperties>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LightDefinitionProperties {
    Basic(BasicLightDefinition),
    Oriented(OrientedLightDefinition),
}

#[derive(Debug, Clone, Deserialize)]
pub struct BasicLightDefinition {
    pub intensity: f32, // Range [0, 1]
    pub size: f32,
    #[serde(default)]
    pub source_orientation_offset: RealOrientation,
    #[serde(default)]
    pub add_perspective: bool,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub minimum_darkness: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrientedLightDefinition {
    #[serde(flatten)]
    pub base: BasicLightDefinition,
    picture: Sprite,
    rotation_shift: Option<RealOrientation>,
}

/// <https://wiki.factorio.com/Types/LightFlickeringDefinition>
#[derive(Debug, Clone, PrototypeFromLua, Deserialize)]
pub struct LightFlickeringDefinition {
    #[serde(default = "default_f32_0_2")]
    pub minimum_intensity: f32, // Default: 0.2 // range [0, 1]
    #[serde(default = "default_f32_0_8")]
    pub maximum_intensity: f32, // Default: 0.8 // range [0, 1]
    #[serde(default = "default_f32_0_3")]
    pub derivation_change_frequency: f32, // Default: 0.3
    #[serde(default = "default_f32_0_06")]
    pub derivation_change_deviation: f32, // Default: 0.06
    #[serde(default = "default_f32_0_02")]
    pub border_fix_speed: f32, // Default: 0.02
    #[serde(default = "default_f32_0_5")]
    pub minimum_light_size: f32, // Default: 0.5
    #[serde(default = "default_f32_0_5")]
    pub light_intensity_to_size_coefficient: f32, // Default: 0.5
    #[serde(default = "default_color_white")]
    pub color: Color, // Default: (1, 1, 1) (White)
}

const fn default_f32_0_2() -> f32 {
    0.2
}
const fn default_f32_0_8() -> f32 {
    0.8
}
const fn default_f32_0_3() -> f32 {
    0.3
}
const fn default_f32_0_06() -> f32 {
    0.06
}
const fn default_f32_0_02() -> f32 {
    0.02
}
const fn default_f32_0_5() -> f32 {
    0.5
}
const fn default_color_white() -> Color {
    Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    }
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#default_recipe_tint>
#[derive(Debug, Clone, Deserialize)]
pub struct RecipeTint {
    // All default to (1, 1, 1, 1), except special cases
    #[serde(default = "default_color_white")]
    pub primary: Color,
    #[serde(default = "default_color_white")]
    pub secondary: Color,
    #[serde(default = "default_color_white")]
    pub tertiary: Color,
    #[serde(default = "default_color_white")]
    pub quaternary: Color,
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#shift_animation_waypoints>
#[derive(Debug, Clone, Deserialize)]
pub struct ShiftAnimationWaypoints {
    pub north: Option<Vec<Factorio2DVector>>,
    pub east: Option<Vec<Factorio2DVector>>,
    pub south: Option<Vec<Factorio2DVector>>,
    pub west: Option<Vec<Factorio2DVector>>,
}

// TODO
/// <https://wiki.factorio.com/Prototype/CraftingMachine#status_colors>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct StatusColors {
    #[default(Color::new_rgb(1.0, 1.0, 1.0))]
    pub idle: Color, // Default: White (1, 1, 1)
    #[default(idle.clone())]
    pub no_minable_resources: Color, // Default: `idle`
    #[default(idle.clone())]
    pub full_output: Color, // Default: `idle`
    #[default(idle.clone())]
    pub insufficient_output: Color, // Default: `idle`
    #[default(idle.clone())]
    pub disabled: Color, // Default: `idle`
    #[default(Color(0.0, 0.0, 0.0, 0.0))]
    pub no_power: Color, // Default: No color
    #[default(Color::new_rgb(1.0, 1.0, 1.0))]
    pub working: Color, // Default: White (1, 1, 1)
    #[default(working.clone())]
    pub low_power: Color, // Default: `working`
}

/// <https://wiki.factorio.com/Types/MiningDrillGraphicsSet#circuit_connector_secondary_draw_order>
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "CircuitConnectorSecondaryDrawOrderIntermediate")]
pub struct CircuitConnectorSecondaryDrawOrder {
    pub north: i8,
    pub east: i8,
    pub south: i8,
    pub west: i8,
}

impl CircuitConnectorSecondaryDrawOrder {
    pub fn new(draw_order: i8) -> Self {
        Self {
            north: draw_order,
            east: draw_order,
            south: draw_order,
            west: draw_order,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CircuitConnectorSecondaryDrawOrderIntermediate {
    SingleNumber(i8),
    Table {
        #[serde(default = "default_i8::<100>")]
        north: i8,
        #[serde(default = "default_i8::<100>")]
        east: i8,
        #[serde(default = "default_i8::<100>")]
        south: i8,
        #[serde(default = "default_i8::<100>")]
        west: i8,
    },
}

impl From<CircuitConnectorSecondaryDrawOrderIntermediate> for CircuitConnectorSecondaryDrawOrder {
    fn from(value: CircuitConnectorSecondaryDrawOrderIntermediate) -> Self {
        match value {
            CircuitConnectorSecondaryDrawOrderIntermediate::SingleNumber(n) => Self::new(n),
            CircuitConnectorSecondaryDrawOrderIntermediate::Table {
                north,
                east,
                south,
                west,
            } => Self {
                north,
                east,
                south,
                west,
            },
        }
    }
}

// TODO
/// <https://wiki.factorio.com/Prototype/Entity#radius_visualisation_specification>
/// <https://wiki.factorio.com/Types/RadiusVisualisationSpecification>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct RadiusVisualizationSpecification {
    pub sprite: Option<Sprite>,
    #[default(0)]
    pub distance: f64, // Default: 0 // Must be >= 0
    pub offset: Option<Factorio2DVector>,
    #[default(true)]
    pub draw_in_cursor: bool, // Default: true
    #[default(true)]
    pub draw_on_selection: bool, // Default: true
}

impl RadiusVisualizationSpecification {
    fn post_extr_fn(
        &mut self,
        _lua: &mlua::Lua,
        _data_table: &DataTable,
    ) -> mlua::prelude::LuaResult<()> {
        if self.distance.is_sign_negative() {
            return Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "RadiusVisualizationSpecification",
                message: Some("`distance` must be positive (>= 0)".into()),
            });
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/WaterReflectionDefinition>
#[derive(Debug, Clone, Deserialize)]
pub struct WaterReflectionDefinition {
    pictures: Option<Vec<SpriteVariation>>,
    #[serde(default)]
    orientation_to_variation: bool, // default: false
    #[serde(default)]
    rotate: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/Sprite#slice_or_dice>
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct Dice(i16, i16);

impl Dice {
    pub fn new(n: i16) -> Self {
        Self(n, n)
    }

    pub fn new_xy(x: i16, y: i16) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum SliceOrDice {
    Square(SliceOrDiceSquare),
    Rectangular(SliceOrDiceNonSquare),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SliceOrDiceSquare {
    Slice(SpriteSizeType),
    Dice(SpriteSizeType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum SliceOrDiceNonSquare {
    Slice {
        slice_x: SpriteSizeType,
        slice_y: SpriteSizeType,
    },
    Dice {
        dice_x: SpriteSizeType,
        dice_y: SpriteSizeType,
    },
}

// ============= // Animations // ============= //

/// <https://wiki.factorio.com/Types/Animation#layers>
#[derive(Debug, Clone)]
pub enum Animation {
    Layers(Vec<Animation>),
    Single(Box<AnimationBase>),
}

impl Animation {
    fn check_stripes(&self) -> bool {
        match self {
            Self::Layers(layers) => {
                let mut flag = false;
                for layer in layers {
                    flag |= layer.check_stripes()
                }
                flag
            }
            Self::Single(ab) => ab.check_stripes(),
        }
    }
}

impl<'lua> PrototypeFromLua<'lua> for Animation {
    fn prototype_from_lua(
        value: mlua::Value<'lua>,
        lua: &'lua mlua::Lua,
        data_table: &mut DataTable,
    ) -> mlua::Result<Self> {
        let type_name = &value.type_name();
        if let mlua::Value::Table(p_table) = value {
            let layers = p_table.get::<_, Option<Vec<mlua::Value>>>("layers")?;
            let result = if let Some(actual_layers) = layers {
                Self::Layers(
                    actual_layers
                        .into_iter()
                        .map(|v| Self::prototype_from_lua(v, lua, data_table))
                        .collect::<mlua::Result<Vec<Self>>>()?,
                )
            } else {
                Self::Single(Box::new(AnimationBase::prototype_from_lua(
                    p_table.to_lua(lua)?,
                    lua,
                    data_table,
                )?))
            };
            if result.check_stripes() {
                return Err(mlua::Error::FromLuaConversionError {
                    from: type_name,
                    to: "Animation",
                    message: Some("`height_in_frames` in stripes is mandatory".into()),
                });
            };
            Ok(result)
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: type_name,
                to: "Animation",
                message: Some("Expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/Animation#hr_version>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct AnimationBase {
    #[use_self_forced]
    pub regular: AnimationSpec,
    pub hr_version: Option<AnimationSpec>,
}

impl AnimationBase {
    fn check_stripes(&self) -> bool {
        self.regular.check_stripes() || {
            if let Some(ans) = &self.hr_version {
                ans.check_stripes()
            } else {
                false
            }
        }
    }
}

/// <https://wiki.factorio.com/Types/Animation>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::register_resources)]
pub struct AnimationSpec {
    // These types share same fields/values, so I decided to "combine" them
    #[mandatory_if(stripes.is_none())]
    pub filename: Option<FileName>,
    #[use_self_forced]
    pub sprite: SpriteSpecWithoutFilename, // Filename is mandatory unless `stripes` is specified
    #[default(RunMode::Forward)]
    pub run_mode: RunMode, // Default: "forward"
    #[default(1_u32)]
    pub frame_count: u32, // Default: 1, can't be 0
    #[default(0_u32)]
    pub line_length: u32, // Default: 0
    #[default(1.0_f32)]
    pub animation_speed: f32, // Default: 1.0
    #[default(f32::MAX)]
    pub max_advance: f32, // Default: MAX_FLOAT
    #[default(1_u8)]
    pub repeat_count: u8, // Default: 1, can't be 0
    pub frame_sequence: Option<AnimationFrameSequence>,
    pub stripes: Option<Vec<Stripe>>,
}

impl AnimationSpec {
    // TODO: clarify the required image sizes for stripes
    fn register_resources(
        &self,
        _lua: &mlua::Lua,
        _data_table: &mut DataTable,
    ) -> mlua::prelude::LuaResult<()> {
        todo!() // TODO
                // List of things to do:
                // Rename this function to a more fitting name
                // check data: `frame_count`, `repeat_count`
    }

    fn check_stripes(&self) -> bool {
        let mut flag = false;
        if let Some(stripes) = &self.stripes {
            for stripe in stripes {
                flag |= stripe.height_in_frames.is_none()
            }
        };
        flag
    }
}

/// <https://wiki.factorio.com/Types/Stripe>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct Stripe {
    pub width_in_frames: u32,
    pub height_in_frames: Option<u32>, // Optional only in RotatedAnimation
    pub filename: FileName,
    #[default(0_u32)]
    pub x: u32, // Default: 0
    #[default(0_u32)]
    pub y: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/AnimationVariations>
pub type AnimationVariations = Vec<AnimationVariation>;

/// <https://wiki.factorio.com/Types/AnimationVariations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct AnimationVariation {
    #[use_self_forced]
    pub animation: AnimationBase, // Filename is mandatory
    pub variation_count: u32,
    #[default(1_u32)]
    pub frame_count: u32, // Default: 1
    #[default(variation_count)]
    pub line_length: u32, // Default: variation_count
}

/// <https://wiki.factorio.com/Types/Animation4Way>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct Animation4Way {
    // All fancy shenanigans are omitted, this program/library behaves like a game
    #[use_self]
    pub north: Animation,
    #[mandatory_if(prot_table.get::<_, Option<Value>>("north")?.is_some())]
    pub east: Option<Animation>,
    #[mandatory_if(prot_table.get::<_, Option<Value>>("north")?.is_some())]
    pub south: Option<Animation>,
    #[mandatory_if(prot_table.get::<_, Option<Value>>("north")?.is_some())]
    pub west: Option<Animation>,
}

/// <https://wiki.factorio.com/Types/AnimationElement>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct AnimationElement {
    #[default(RenderLayer::Object)]
    pub render_layer: RenderLayer, // Default: "object"
    pub secondary_draw_order: Option<i8>,
    #[default(true)]
    pub draw_as_sprite: bool, // Default: true
    #[default(false)]
    pub draw_as_light: bool, // Default: false
    #[default(false)]
    pub apply_tint: bool, // Default: false
    #[default(true)]
    pub always_draw: bool, // Default: true
    pub animation: Animation,
}

/// <https://wiki.factorio.com/Types/RotatedAnimation>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedAnimation {
    #[use_self_forced]
    pub regular: RotatedAnimationSpec,
    pub hr_version: Option<RotatedAnimationSpec>,
}

// A: "Are you sure this will work?"; Me: "I have no idea!"
// Don't forget to check Stripes to set `height_in_frames` to `direction_count` if it's None
/// <https://wiki.factorio.com/Types/RotatedAnimation>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedAnimationSpec {
    pub direction_count: u32,
    #[default(0_u32)]
    pub still_frame: u32, // Default: 0
    #[default(false)]
    pub axially_symmetrical: bool, // Default: false
    #[default(false)]
    pub counterclockwise: bool, // Default: false
    #[default(0.5_f32)]
    pub middle_orientation: f32, // Default: 0.5
    #[default(1_f32)]
    pub orientation_range: f32, // Default: 1
    #[default(true)]
    pub apply_projection: bool, // Default: true
    #[use_self_forced] // TODO
    pub animation: AnimationVariation,
}

/// <https://wiki.factorio.com/Types/RotatedAnimationVariations>
pub type RotatedAnimationVariations = Vec<RotatedAnimationVariation>;

/// <https://wiki.factorio.com/Types/RotatedAnimationVariations>
#[derive(Debug, Clone)]
pub enum RotatedAnimationVariation {
    Layers(Vec<RotatedAnimationVariation>),
    Single(Box<RotatedAnimation>),
}

impl<'lua> PrototypeFromLua<'lua> for RotatedAnimationVariation {
    fn prototype_from_lua(
        value: mlua::Value<'lua>,
        lua: &'lua mlua::Lua,
        data_table: &mut DataTable,
    ) -> mlua::Result<Self> {
        let type_name = value.type_name();
        if let Value::Table(t) = value {
            if t.get::<_, Option<String>>("direction_count")?.is_some() {
                Ok(Self::Single(Box::new(
                    RotatedAnimation::prototype_from_lua(t.to_lua(lua)?, lua, data_table)?,
                )))
            } else {
                Ok(Self::Layers(
                    t.sequence_values::<Value>()
                        .map(|v| RotatedAnimationVariation::prototype_from_lua(v?, lua, data_table))
                        .collect::<LuaResult<Vec<RotatedAnimationVariation>>>()?,
                ))
            }
        } else {
            Err(LuaError::FromLuaConversionError {
                from: type_name,
                to: "RotatedAnimationVariation",
                message: Some("expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/RotatedAnimation4Way>
#[derive(Debug, Clone)]
pub struct RotatedAnimation4Way {
    north: RotatedAnimation,
    east: RotatedAnimation,
    // Next 2 are optional, north and west are used if these are not present
    south: RotatedAnimation,
    west: RotatedAnimation,
}

/// <https://wiki.factorio.com/Prototype/Beam#light_animations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct LightAnimations {
    pub start: Option<Animation>,
    pub ending: Option<Animation>,
    pub head: Option<Animation>,
    pub tail: Option<Animation>,
    pub body: Option<Vec<AnimationVariation>>,
}

// ============== // Sprites // ==============  //

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct Sprite {
    #[use_self_vec]
    pub layers: Vec<SpriteLayer>,
}

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteLayer {
    #[use_self_forced]
    pub regular: SpriteSpec,
    pub hr_version: Option<SpriteSpec>,
}

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteSpec {
    pub filename: FileName, // Mandatory in some cases
    #[use_self_forced]
    pub body: SpriteSpecWithoutFilename,
}

#[derive(Debug, Clone)]
pub struct SpriteSpecWithoutFilename {
    pub dice: Option<SliceOrDice>, // AKA slice // _y and _x are converted into this
    pub priority: SpritePriority,  // Default: "medium"
    pub flags: Option<SpriteFlags>,
    pub size: SpriteSize,
    // Automatically converted to size
    // width
    // height
    pub position: Option<SpritePosition>,
    // Automatically converted to position
    // x
    // y
    pub shift: Factorio2DVector,    // (0, 0) by default
    pub scale: f64,                 // 1 by default,
    pub draw_as: DrawAs,            // all false by default
    pub mipmap_count: u8,           // Default: 0
    pub apply_runtime_tint: bool,   // Default: false
    pub tint: Color,                // Default: (1, 1, 1, 1) (white)
    pub blend_mode: BlendMode,      // Default: "normal"
    pub load_in_minimal_mode: bool, //Default: false
    pub premul_alpha: bool,         // Default: true
    pub generate_sfd: bool, // Default: false // Only used by sprites in UtilitySprites with "icon" flag
}

// TODO: do this with a macro

impl<'lua> PrototypeFromLua<'lua> for SpriteSpecWithoutFilename {
    fn prototype_from_lua(
        value: mlua::Value<'lua>,
        lua: &'lua mlua::Lua,
        _data_table: &mut DataTable,
    ) -> mlua::Result<Self> {
        if let mlua::Value::Table(p_table) = value {
            let dice: Option<Dice> = {
                let dice_gen_opt: Option<i16> = p_table
                    .get::<_, Option<i16>>("dice")?
                    .or_else(|| p_table.get("slice").ok());
                if let Some(dice_gen) = dice_gen_opt {
                    Some(Dice::new(dice_gen))
                } else {
                    let x: Option<i16> = p_table
                        .get::<_, Option<i16>>("dice_x")?
                        .or_else(|| p_table.get("slice_x").ok());
                    let y: Option<i16> = p_table
                        .get::<_, Option<i16>>("dice_y")?
                        .or_else(|| p_table.get("slice_y").ok());
                    if let (Some(ax), Some(ay)) = (x, y) {
                        Some(Dice(ax, ay))
                    } else {
                        None
                    }
                }
            };
            let priority: SpritePriority = p_table
                .get::<_, Option<String>>("priority")?
                .unwrap_or_else(|| "medium".into())
                .parse()
                .map_err(|_| mlua::Error::FromLuaConversionError {
                    from: "String",
                    to: "SpritePriority",
                    message: Some("invalid value".into()),
                })?;
            let flags = p_table
                .get::<_, Option<Vec<String>>>("flags")?
                .map(SpriteFlags::from_iter);
            let size = {
                let error = Err(mlua::Error::FromLuaConversionError {
                    from: "integer",
                    to: "SpriteSizeType",
                    message: Some("value must be in range 0-8192".into()),
                });
                if let Some(s_value) = p_table.get::<_, Option<mlua::Value>>("size")? {
                    match lua.unpack::<i16>(s_value.clone()) {
                        Ok(size) => {
                            if (0..=8192_i16).contains(&size) {
                                SpriteSize(size, size)
                            } else {
                                return error;
                            }
                        }
                        _ => {
                            let size = lua.unpack::<[i16; 2]>(s_value)?;
                            if (0..=8192_i16).contains(&size[0])
                                && (0..=8192_i16).contains(&size[1])
                            {
                                SpriteSize(size[0], size[1])
                            } else {
                                return error;
                            }
                        }
                    }
                } else {
                    let width: SpriteSizeType = p_table.get("width")?;
                    let height: SpriteSizeType = p_table.get("height")?;
                    if (0..=8192_i16).contains(&width) && (0..=8192_i16).contains(&height) {
                        SpriteSize(width, height)
                    } else {
                        return error;
                    }
                }
            };
            let position = {
                let x = p_table.get::<_, Option<SpriteSizeType>>("x")?.unwrap_or(0);
                let y = p_table.get::<_, Option<SpriteSizeType>>("y")?.unwrap_or(0);
                if x != 0 || y != 0 {
                    Some(SpritePosition(x, y))
                } else {
                    p_table
                        .get::<_, Option<[SpriteSizeType; 2]>>("position")?
                        .map(|pos| SpritePosition(pos[0], pos[1]))
                }
            };
            let shift = p_table
                .get::<_, Option<Factorio2DVector>>("shift")?
                .unwrap_or(Factorio2DVector(0.0, 0.0));
            let scale = p_table.get::<_, Option<f64>>("scale")?.unwrap_or(1.0);
            let draw_as = {
                let draw_as_shadow = p_table
                    .get::<_, Option<bool>>("draw_as_shadow")?
                    .unwrap_or(false);
                let draw_as_glow = p_table
                    .get::<_, Option<bool>>("draw_as_glow")?
                    .unwrap_or(false);
                let draw_as_light = p_table
                    .get::<_, Option<bool>>("draw_as_light")?
                    .unwrap_or(false);
                DrawAs::new(draw_as_shadow, draw_as_glow, draw_as_light)
            };
            let mipmap_count = p_table.get::<_, Option<u8>>("mipmap_count")?.unwrap_or(0);
            let apply_runtime_tint = p_table
                .get::<_, Option<bool>>("apply_runtime_tint")?
                .unwrap_or(false);
            let tint = p_table
                .get::<_, Option<Color>>("tint")?
                .unwrap_or(Color(1.0, 1.0, 1.0, 1.0));
            let blend_mode: BlendMode = p_table
                .get::<_, Option<String>>("blend_mode")?
                .unwrap_or_else(|| "normal".into())
                .parse()
                .map_err(|_| mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: "BlendMode",
                    message: Some("Invalid variant".into()),
                })?;
            let load_in_minimal_mode = p_table
                .get::<_, Option<bool>>("load_in_minimal_mode")?
                .unwrap_or(false);
            let premul_alpha = p_table
                .get::<_, Option<bool>>("premul_alpha")?
                .unwrap_or(true);
            let generate_sfd = p_table
                .get::<_, Option<bool>>("generate_sfd")?
                .unwrap_or(false);
            Ok(Self {
                dice,
                priority,
                flags,
                size,
                position,
                shift,
                scale,
                draw_as,
                mipmap_count,
                apply_runtime_tint,
                tint,
                blend_mode,
                load_in_minimal_mode,
                premul_alpha,
                generate_sfd,
            })
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "SpriteSpec",
                message: Some("Expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/SpriteNWaySheet>
#[derive(Debug, Clone)]
pub struct SpriteNWaySheet {
    pub sprite: SpriteSpec,
    pub frames: u32, // 4 or 8
}

impl SpriteNWaySheet {
    fn new<'lua>(
        value: mlua::Value<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
        frames: u32,
    ) -> LuaResult<Self> {
        if let mlua::Value::Table(t) = &value {
            let frames = t.get::<_, Option<u32>>("frames")?.unwrap_or(frames);
            let sprite = SpriteSpec::prototype_from_lua(value, lua, data_table)?;
            Ok(Self { sprite, frames })
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "SpriteNWaySheet",
                message: Some("Expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/Sprite4Way>
#[derive(Debug, Clone)]
pub struct Sprite4Way(pub DirectionalSprite);

impl<'lua> PrototypeFromLua<'lua> for Sprite4Way {
    fn prototype_from_lua(
        value: LuaValue<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        let type_name = value.type_name();
        if let LuaValue::Table(t) = &value {
            if let Some(sheets) = t.get::<_, Option<Vec<Value>>>("sheets")? {
                let sheets = sheets
                    .into_iter()
                    .map(|v| SpriteNWaySheet::new(v, lua, data_table, 4))
                    .collect::<LuaResult<Vec<SpriteNWaySheet>>>()?;
                Ok(Self(sheets.into()))
            } else if let Some(sheet) = t.get::<_, Option<Value>>("sheet")? {
                let sheets = vec![SpriteNWaySheet::new(sheet, lua, data_table, 4)?];
                Ok(Self(sheets.into()))
            } else {
                Ok(Self(DirectionalSprite::Directions(
                    SpriteDirections::prototype_from_lua(value, lua, data_table)?,
                )))
            }
        } else {
            Err(LuaError::FromLuaConversionError {
                from: type_name,
                to: "Sprite8Way",
                message: Some("expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/Sprite8Way>
#[derive(Debug, Clone)]
pub struct Sprite8Way(pub DirectionalSprite);

impl<'lua> PrototypeFromLua<'lua> for Sprite8Way {
    fn prototype_from_lua(
        value: LuaValue<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        let type_name = value.type_name();
        if let LuaValue::Table(t) = &value {
            if let Some(sheets) = t.get::<_, Option<Vec<Value>>>("sheets")? {
                let sheets = sheets
                    .into_iter()
                    .map(|v| SpriteNWaySheet::new(v, lua, data_table, 8))
                    .collect::<LuaResult<Vec<SpriteNWaySheet>>>()?;
                Ok(Self(sheets.into()))
            } else if let Some(sheet) = t.get::<_, Option<Value>>("sheet")? {
                let sheets = vec![SpriteNWaySheet::new(sheet, lua, data_table, 8)?];
                Ok(Self(sheets.into()))
            } else {
                Ok(Self(DirectionalSprite::Directions(
                    SpriteDirections::prototype_from_lua(value, lua, data_table)?,
                )))
            }
        } else {
            Err(LuaError::FromLuaConversionError {
                from: type_name,
                to: "Sprite8Way",
                message: Some("expected table".into()),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirectionalSprite {
    Sheets(Vec<SpriteNWaySheet>),
    Directions(SpriteDirections),
}

impl From<Vec<SpriteNWaySheet>> for DirectionalSprite {
    fn from(sheets: Vec<SpriteNWaySheet>) -> Self {
        Self::Sheets(sheets)
    }
}

#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteDirections {
    pub north: Option<Sprite>,
    pub north_east: Option<Sprite>,
    pub east: Option<Sprite>,
    pub south_east: Option<Sprite>,
    pub south: Option<Sprite>,
    pub south_west: Option<Sprite>,
    pub west: Option<Sprite>,
    pub north_west: Option<Sprite>,
}

/// <https://wiki.factorio.com/Types/RotatedSprite#layers>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedSprite {
    #[use_self_vec]
    pub layers: Vec<RotatedSpriteLayer>,
}

/// <https://wiki.factorio.com/Types/RotatedSprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedSpriteLayer {
    #[use_self_forced]
    pub regular: RotatedSpriteSpec,
    pub hr_version: Option<RotatedSpriteSpec>,
}

/// <https://wiki.factorio.com/Types/RotatedSprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedSpriteSpec {
    pub sprites: Vec<SpriteSpec>, // If `filenames` is set, copy all properties to each object for each filename // FIXME
    pub direction_count: u16,
}

/// <https://wiki.factorio.com/Types/SpriteVariations>
pub type SpriteVariations = Vec<SpriteVariation>;

/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteVariation {
    #[use_self_vec]
    pub layers: Vec<SpriteVariationLayer>,
}

/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteVariationLayer {
    #[use_self_forced]
    pub regular: SpriteVariationSpec,
    pub hr_version: Option<SpriteVariationSpec>,
}

/// Extension of SpriteSpec, ignores dice and slice
/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteVariationSpec {
    #[use_self_forced]
    pub sprite: SpriteSpec,
    #[default(1_u32)]
    pub variation_count: u32, // Default: 1
    #[default(1_u32)]
    pub repeat_count: u32, // Default: 1
    #[default(variation_count)]
    pub line_length: u32, // Default: value of `variation_count`
}

/// <https://wiki.factorio.com/Types/SpriteFlags>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
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

impl<T: AsRef<str>> FromIterator<T> for SpriteFlags {
    fn from_iter<I: IntoIterator<Item = T>>(flags: I) -> Self {
        let mut result = Self(0);
        for flag in flags {
            match flag.as_ref() {
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
                "icon" => {
                    result |= SpriteFlags::ICON
                        | SpriteFlags::NO_CROP
                        | SpriteFlags::NO_SCALE
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::LINEAR_MAGNIFICATION
                        | SpriteFlags::LINEAR_MIP_LEVEL
                        | SpriteFlags::NOT_COMPRESSED
                        | SpriteFlags::GROUP_ICON
                }
                "gui" => {
                    result |= SpriteFlags::GUI
                        | SpriteFlags::NO_CROP
                        | SpriteFlags::NO_SCALE
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::LINEAR_MAGNIFICATION
                        | SpriteFlags::LINEAR_MIP_LEVEL
                        | SpriteFlags::NOT_COMPRESSED
                        | SpriteFlags::GROUP_GUI
                }
                "gui-icon" => {
                    result |= SpriteFlags::GUI_ICON
                        | SpriteFlags::NO_CROP
                        | SpriteFlags::NO_SCALE
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::LINEAR_MAGNIFICATION
                        | SpriteFlags::NOT_COMPRESSED
                        | SpriteFlags::GROUP_ICON
                }
                "light" => {
                    result |= SpriteFlags::LIGHT
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MIP_LEVEL
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::LINEAR_MAGNIFICATION
                        | SpriteFlags::GROUP_NONE
                }
                "terrain" => {
                    result |= SpriteFlags::TERRAIN
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MIP_LEVEL
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::NO_CROP
                        | SpriteFlags::GROUP_TERRAIN
                }
                "terrain-effect-map" => {
                    result |= SpriteFlags::TERRAIN_EFFECT_MAP
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MIP_LEVEL
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::NO_CROP
                        | SpriteFlags::GROUP_TERRAIN_EFFECT_MAP
                }
                "shadow" => result |= SpriteFlags::SHADOW,
                "smoke" => {
                    result |= SpriteFlags::SMOKE
                        | SpriteFlags::MIPMAP
                        | SpriteFlags::LINEAR_MINIFICATION
                        | SpriteFlags::LINEAR_MAGNIFICATION
                        | SpriteFlags::GROUP_SMOKE
                }
                "decal" => result |= SpriteFlags::DECAL | SpriteFlags::GROUP_DECAL,
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
                "group=icon-background" => result |= SpriteFlags::GROUP_ICON_BACKGROUND,
                */
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
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SpritePriority {
    ExtraHighNoScale,
    ExtraHigh,
    High,
    Medium,
    Low,
    VeryLow,
    NoAtlas,
}

// ===== // Graphics Sets and Pictures // ===== //

/// <https://wiki.factorio.com/Prototype/Rail#pictures>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RailPictures {
    pub straight_rail_horizontal: RailPieceLayers,
    pub straight_rail_vertical: RailPieceLayers,
    pub straight_rail_diagonal_left_top: RailPieceLayers,
    pub straight_rail_diagonal_right_top: RailPieceLayers,
    pub straight_rail_diagonal_right_bottom: RailPieceLayers,
    pub straight_rail_diagonal_left_bottom: RailPieceLayers,
    pub curved_rail_vertical_left_top: RailPieceLayers,
    pub curved_rail_vertical_right_top: RailPieceLayers,
    pub curved_rail_vertical_right_bottom: RailPieceLayers,
    pub curved_rail_vertical_left_bottom: RailPieceLayers,
    pub curved_rail_horizontal_left_top: RailPieceLayers,
    pub curved_rail_horizontal_right_top: RailPieceLayers,
    pub curved_rail_horizontal_right_bottom: RailPieceLayers,
    pub curved_rail_horizontal_left_bottom: RailPieceLayers,
    pub rail_endings: Sprite8Way,
}

/// <https://wiki.factorio.com/Types/RailPieceLayers>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RailPieceLayers {
    // TODO: checks
    pub metals: Vec<SpriteVariation>,
    pub backplayes: Vec<SpriteVariation>, // Must have same number of variations as `metals`
    pub ties: Vec<SpriteVariation>,       // Must have between 1 and 4 items
    pub stone_path: Vec<SpriteVariation>, // Must have between 1 and 4 items
    pub stone_path_background: Option<Vec<SpriteVariation>>,
    pub segment_visualisation_middle: Option<Sprite>,
    pub segment_visualisation_ending_front: Option<Sprite>,
    pub segment_visualisation_ending_back: Option<Sprite>,
    pub segment_visualisation_continuing_front: Option<Sprite>,
    pub segment_visualisation_continuing_back: Option<Sprite>,
}

/// <https://wiki.factorio.com/Types/CircuitConnectorSprites>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct CircuitConnectorSprites {
    pub led_red: Sprite,
    pub led_green: Sprite,
    pub led_blue: Sprite,
    pub led_light: LightDefinition,
    pub connector_main: Option<Sprite>,
    pub connector_shadow: Option<Sprite>,
    pub wire_pins: Option<Sprite>,
    pub wire_pins_shadow: Option<Sprite>,
    pub led_blue_off: Option<Sprite>,
    pub blue_led_light_offset: Option<Factorio2DVector>,
    pub red_green_led_light_offset: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct BeaconGraphicsSet {
    #[default(true)]
    pub draw_animation_when_idle: bool, //Default: true
    #[default(false)]
    pub draw_light_when_idle: bool, // Default: false
    #[default(false)]
    pub random_animation_offset: bool, // Default: false
    #[default(false)]
    pub module_icons_suppressed: bool, // Default: false
    #[default(RenderLayer::Object)]
    pub base_layer: RenderLayer, // Default: "object"
    #[default(RenderLayer::Object)]
    pub animation_layer: RenderLayer, // Default: "object"
    #[default(RenderLayer::Object)]
    pub top_layer: RenderLayer, // Default: "object"
    #[default(1_f32)]
    pub animation_progress: f32, // Default: 1
    #[default(0_f32)]
    pub min_animation_progress: f32, // Default: 0
    #[default(1000_f32)]
    pub max_animation_progress: f32, // Default: 1000
    #[default(ApplyModuleTint::None)]
    pub apply_module_tint: ApplyModuleTint, // Default: "none"
    #[default(ApplyModuleTint::None)]
    pub apply_module_tint_to_light: ApplyModuleTint, // Default: "none"
    #[default(Color(0.0, 0.0, 0.0, 1.0))]
    pub no_modules_tint: Color, //Default: no color
    pub animation_list: Option<Vec<AnimationElement>>,
    pub light: Option<LightDefinition>,
    pub module_visualisations: Option<BeaconModuleVisualizations>,
    #[default(ModuleTintMode::SingleModule)]
    pub module_tint_mode: ModuleTintMode, // Default: "single-module"
}

/// <https://wiki.factorio.com/Types/BeaconModuleVisualizations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct BeaconModuleVisualizations {
    pub art_style: String,
    #[default(false)]
    pub use_for_empty_slots: bool, // Default: false
    #[default(0_i32)]
    pub tier_offset: i32, // Default: 0
    pub slots: Option<Vec<Vec<BeaconModuleVisualization>>>,
}

/// <https://wiki.factorio.com/Types/BeaconModuleVisualization>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct BeaconModuleVisualization {
    #[default(false)]
    pub has_empty_slot: bool, // Default: false
    #[default(false)]
    pub draw_as_light: bool, // Default: false
    #[default(true)]
    pub draw_as_sprite: bool, // Default: true
    #[default(0_i8)]
    pub secondary_draw_order: i8, // Default: 0
    #[default(ApplyModuleTint::None)]
    pub apply_module_tint: ApplyModuleTint, // Default: "none"
    #[default(RenderLayer::Object)]
    pub render_layer: RenderLayer, // Default: "object"
    pub pictures: Option<Vec<SpriteVariation>>,
}

/// <https://wiki.factorio.com/Types/CharacterArmorAnimation>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct CharacterArmorAnimation {
    pub idle: RotatedAnimation,
    pub idle_with_gun: RotatedAnimation,
    pub running: RotatedAnimation,
    pub running_with_gun: RotatedAnimation,
    pub mining_with_tool: RotatedAnimation,
    pub flipped_shadow_running_with_gun: Option<RotatedAnimation>, // If set, must containt exactly 18 directions // TODO
    pub armors: Option<Vec<String>>,
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct WorkingVisualisation {
    #[default(RenderLayer::Object)]
    pub render_layer: RenderLayer, // Default: "object"
    #[default(false)]
    pub fadeout: bool, // Default: false
    #[default(false)]
    pub synced_fadeout: bool, // Default: false
    #[default(false)]
    pub constant_speed: bool, // Default: false
    #[default(false)]
    pub always_draw: bool, // Default: false
    #[default(false)]
    pub animated_shift: bool, // Default: false
    #[default(false)]
    pub align_to_waypoint: bool, // Default: false
    pub secondary_draw_order: Option<i8>,
    #[default(true)]
    pub draw_as_sprite: bool, // Default: true
    #[default(false)]
    pub draw_as_light: bool, // Default: false
    pub light: Option<LightDefinition>,
    pub effect: Option<WorkingVisualisationEffect>,
    pub apply_recipe_tint: Option<ApplyRecipeTint>,
    pub apply_tint: Option<ApplyTint>,
    pub north_animation: Option<Animation>,
    pub west_animation: Option<Animation>,
    pub south_animation: Option<Animation>,
    pub east_animation: Option<Animation>,
    pub animation: Option<Animation>,
    pub north_position: Option<Factorio2DVector>,
    pub west_position: Option<Factorio2DVector>,
    pub south_position: Option<Factorio2DVector>,
    pub east_position: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation#effect>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum WorkingVisualisationEffect {
    Flicker,
    UraniumGlow,
    None,
}

impl<'lua> PrototypeFromLua<'lua> for WorkingVisualisationEffect {
    fn prototype_from_lua(
        value: LuaValue<'lua>,
        lua: &'lua Lua,
        _data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        let s: String = lua.unpack(value)?;
        s.parse().map_err(LuaError::external)
    }
}

/// <https://wiki.factorio.com/Types/ConnectableEntityGraphics>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ConnectableEntityGraphics {
    pub single: Vec<SpriteVariation>,
    pub straight_vertical: Vec<SpriteVariation>,
    pub straight_horizontal: Vec<SpriteVariation>,
    pub corner_right_down: Vec<SpriteVariation>,
    pub corner_left_down: Vec<SpriteVariation>,
    pub corner_right_up: Vec<SpriteVariation>,
    pub corner_left_up: Vec<SpriteVariation>,
    pub t_up: Vec<SpriteVariation>,
    pub t_right: Vec<SpriteVariation>,
    pub t_down: Vec<SpriteVariation>,
    pub t_left: Vec<SpriteVariation>,
    pub ending_up: Vec<SpriteVariation>,
    pub ending_right: Vec<SpriteVariation>,
    pub ending_down: Vec<SpriteVariation>,
    pub ending_left: Vec<SpriteVariation>,
    pub cross: Vec<SpriteVariation>,
}

/// <https://wiki.factorio.com/Types/MiningDrillGraphicsSet>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct MiningDrillGraphicsSet {
    pub animation: Option<Animation4Way>,
    pub idle_animation: Option<Animation4Way>,
    #[default(false)]
    pub always_draw_idle_animation: bool, // Default: false
    pub default_recipe_tint: Option<RecipeTint>,
    pub working_visualisations: Option<WorkingVisualisation>,
    /// Only loaded if `shift_animation_waypoint_stop_duration` or `shift_animation_transition_duration` is not 0 // TODO
    pub shift_animation_waypoints: Option<ShiftAnimationWaypoints>,
    #[default(0_u16)]
    pub shift_animation_waypoint_stop_duration: u16, // Default: 0
    #[default(0_u16)]
    pub shift_animation_transition_duration: u16, // Default: 0
    pub status_colors: Option<StatusColors>,
    #[default(0_u16)]
    pub drilling_vertical_movement_duration: u16, // Default: 0
    #[default(1_f32)]
    pub animation_progress: f32, // Default: 1
    #[default(1000_f32)]
    pub max_animation_progress: f32, // Default: 1000
    #[default(0_f32)]
    pub min_animation_progress: f32, // Default: 0
    pub circuit_connector_layer: CircuitConnectorRenderLayers, // Default: all "object" // TODO: load as string
    pub circuit_connector_secondary_draw_order: CircuitConnectorSecondaryDrawOrder, // Default: all 100
}

/// <https://wiki.factorio.com/Types/MiningDrillGraphicsSet#circuit_connector_layer>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct CircuitConnectorRenderLayers {
    pub north: RenderLayer,
    pub east: RenderLayer,
    pub south: RenderLayer,
    pub west: RenderLayer,
}

impl CircuitConnectorRenderLayers {
    pub fn new(render_layer: RenderLayer) -> Self {
        Self {
            north: render_layer,
            east: render_layer,
            south: render_layer,
            west: render_layer,
        }
    }
}

/// <https://wiki.factorio.com/Prototype/OffshorePump#graphics_set>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct OffshorePumpGraphicsSet {
    pub animation: Animation4Way,
    #[default(RenderLayer::GroundPatch)]
    pub base_render_layer: RenderLayer, // Default: "ground-patch"
    #[default(1_i8)]
    pub underwater_layer_offset: i8, // Default: 1
    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,
    pub base_pictures: Option<Sprite4Way>,
    pub underwater_pictures: Option<Sprite4Way>,
}

/// <https://wiki.factorio.com/Prototype/Pipe#pictures>
#[derive(Debug, Clone)]
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
    gas_flow: Animation,
}

/// <https://wiki.factorio.com/Prototype/PipeToGround#pictures>
#[derive(Debug, Clone)]
pub struct PipeToGroundPictures {
    down: Sprite,
    up: Sprite,
    left: Sprite,
    ritgh: Sprite,
}

/// <https://wiki.factorio.com/Prototype/Pump#fluid_wagon_connector_graphics>
#[derive(Debug, Clone)]
pub struct PumpConnectorGraphicsFluidWagon {
    load_animations: PumpConnectorGraphics,
    unload_animations: PumpConnectorGraphics,
}

/// <https://wiki.factorio.com/Types/PumpConnectorGraphics>
#[derive(Debug, Clone)]
pub struct PumpConnectorGraphics {
    north: Vec<PumpConnectorGraphicsMapping>,
    east: Vec<PumpConnectorGraphicsMapping>,
    south: Vec<PumpConnectorGraphicsMapping>,
    west: Vec<PumpConnectorGraphicsMapping>,
}

/// <https://wiki.factorio.com/Types/PumpConnectorGraphics>
#[derive(Debug, Clone)]
pub struct PumpConnectorGraphicsMapping {
    standup_base: Option<Animation>,
    standup_top: Option<Animation>,
    standup_shadow: Option<Animation>,
    connector: Option<Animation>,
    connector_shadow: Option<Animation>,
}

/// <https://wiki.factorio.com/Prototype/SimpleEntity#pictures>
#[derive(Debug, Clone)]
pub enum SimpleEntityVisuals {
    Pictures(SpriteVariations),
    Picture(Sprite),
    Animations(AnimationVariations),
}

/// <https://wiki.factorio.com/Prototype/SimpleEntityWithOwner#pictures>
#[derive(Debug, Clone)]
pub enum SimpleEntityWithOwnerVisuals {
    Pictires(SpriteVariations),
    Pictire(Sprite4Way),
    Animations(AnimationVariations),
}

/// <https://wiki.factorio.com/Types/SpiderLegGraphicsSet>
#[derive(Debug, Clone)]
pub struct SpiderLegGraphicsSet {
    joint_turn_offset: f32, // Default: 0
    joint: Option<Sprite>,
    joint_shadow: Option<Sprite>,
    upper_part: Option<SpiderLegPart>,
    lower_part: Option<SpiderLegPart>,
    upper_part_shadow: Option<SpiderLegPart>,
    lower_part_shadow: Option<SpiderLegPart>,
    upper_part_water_reflection: Option<SpiderLegPart>,
    lower_part_water_reflection: Option<SpiderLegPart>,
}

/// <https://wiki.factorio.com/Types/SpiderLegPart>
#[derive(Debug, Clone)]
pub struct SpiderLegPart {
    top_end: Option<Sprite>,
    middle: Option<Sprite>,
    bottom_end: Option<Sprite>,
    middle_offset_from_top: f32,    // Default: 0
    middle_offset_from_bottom: f32, // Default: 0
    top_end_length: f32,            // Default: 0
    bottom_end_length: f32,         // Default: 0
}

/// <https://wiki.factorio.com/Prototype/StorageTank#pictures>
#[derive(Debug, Clone)]
pub struct StorageTankPictures {
    picture: Sprite4Way,
    window_background: Sprite,
    fluid_background: Sprite,
    flow_sprite: Sprite,
    gas_flow: Animation,
}

/// <https://wiki.factorio.com/Prototype/TrainStop#light1>
#[derive(Debug, Clone)]
pub struct TrainStopLight {
    sprite: Sprite4Way,
    red_picture: Sprite4Way,
    light: LightDefinition,
}

/// <https://wiki.factorio.com/Prototype/TrainStop#drawing_boxes>
#[derive(Debug, Clone)]
pub struct TrainStopDrawingBoxes {
    north: BoundingBox,
    east: BoundingBox,
    south: BoundingBox,
    west: BoundingBox,
}

/// <https://wiki.factorio.com/Prototype/TransportBeltConnectable#belt_animation_set>
#[derive(Debug, Clone)]
pub struct BeltAnimationSet {
    animation_set: RotatedAnimation,
    east_index: u8,           // Default: 1
    west_index: u8,           // Default: 2
    north_index: u8,          // Default: 3
    south_index: u8,          // Default: 4
    starting_south_index: u8, // Default: 13
    ending_south_index: u8,   // Default: 14
    starting_west_index: u8,  // Default: 15
    ending_west_index: u8,    // Default: 16
    starting_north_index: u8, // Default: 17
    ending_north_index: u8,   // Default: 18
    starting_east_index: u8,  // Default: 19
    ending_east_index: u8,    // Default: 20
    ending_patch: Option<Sprite4Way>,
    ends_with_stopper: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/TransportBeltConnectable#belt_horizontal>
#[derive(Debug, Clone)]
pub struct BeltGraphicsSet {
    belt_horizontal: Animation,
    belt_vertical: Animation,
    ending_top: Animation,
    ending_bottom: Animation,
    ending_side: Animation,
    starting_top: Animation,
    starting_bottom: Animation,
    starting_side: Animation,
    ending_patch: Option<Sprite4Way>,
    ends_with_stopper: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/Loader1x1#structure>
#[derive(Debug, Clone)]
pub struct BeltStructure {
    direction_in: Sprite4Way,
    direction_out: Sprite4Way,
    back_patch: Option<Sprite4Way>,
    front_patch: Option<Sprite4Way>,
}

/// <https://wiki.factorio.com/Prototype/LinkedBelt#structure>
#[derive(Debug, Clone)]
pub struct BeltStructureWithSideLoading {
    base_structure: BeltStructure,
    direction_in_side_loading: Option<Sprite4Way>,
    direction_out_side_loading: Option<Sprite4Way>,
}

/// <https://wiki.factorio.com/Types/TransportBeltConnectorFrame>
#[derive(Debug, Clone)]
pub struct TransportBeltConnectorFrame {
    frame_main: AnimationVariations,
    frame_shadow: AnimationVariations,
    frame_main_scanner: Animation,
    frame_main_scanner_movement_speed: f32,
    frame_main_scanner_horizontal_start_shift: Factorio2DVector,
    frame_main_scanner_horizontal_end_shift: Factorio2DVector,
    frame_main_scanner_horizontal_y_scale: f32,
    frame_main_scanner_horizontal_rotation: RealOrientation,
    frame_main_scanner_vertical_start_shift: Factorio2DVector,
    frame_main_scanner_vertical_end_shift: Factorio2DVector,
    frame_main_scanner_vertical_y_scale: f32,
    frame_main_scanner_vertical_rotation: RealOrientation,
    frame_main_scanner_cross_horizontal_start_shift: Factorio2DVector,
    frame_main_scanner_cross_horizontal_end_shift: Factorio2DVector,
    frame_main_scanner_cross_horizontal_y_scale: f32,
    frame_main_scanner_cross_horizontal_rotation: RealOrientation,
    frame_main_scanner_cross_vertical_start_shift: Factorio2DVector,
    frame_main_scanner_cross_vertical_end_shift: Factorio2DVector,
    frame_main_scanner_cross_vertical_y_scale: f32,
    frame_main_scanner_cross_vertical_rotation: RealOrientation,
    frame_main_scanner_nw_ne: Animation,
    frame_main_scanner_sw_se: Animation,
    frame_back_patch: Option<SpriteVariations>,
    frame_front_patch: Option<SpriteVariations>,
}

/// <https://wiki.factorio.com/Prototype/TransportBelt#belt_animation_set>
#[derive(Debug, Clone)]
pub struct BeltAnimationSetIndexes {
    east_to_north_index: u8, // Default: 5
    north_to_east_index: u8, // Default: 6
    west_to_north_index: u8, // Default: 7
    north_to_west_index: u8, // Default: 8
    south_to_east_index: u8, // Default: 9
    east_to_south_index: u8, // Default: 10
    south_to_west_index: u8, // Default: 11
    west_to_south_index: u8, // Default: 12
}

/// <https://wiki.factorio.com/Prototype/Tree#pictures>
#[derive(Debug, Clone)]
pub enum TreeVisuals {
    Pictures(TreePictures),
    Variations(Vec<TreePrototypeVariation>), // Non-empty array
}

/// <https://wiki.factorio.com/Prototype/Tree#pictures>
#[derive(Debug, Clone)]
pub struct TreePictures {
    pictures: SpriteVariations,
    color: Vec<Color>,
}

/// <https://wiki.factorio.com/Prototype/Tree#variations>
#[derive(Debug, Clone)]
pub struct TreePrototypeVariation {
    trunk: Animation,
    leaves: Animation,
    leaf_generation: CreateParticleTriggerEffectItem,
    branch_generation: CreateParticleTriggerEffectItem,
    shadow: Option<Animation>,
    disable_shadow_distortion_beginning_at_frame: u32, // Default: shadow.frame_count - 1
    normal: Option<Animation>,
    overlay: Option<Animation>,
    water_reflection: Option<WaterReflectionDefinition>,
}

/// <https://wiki.factorio.com/Types/SpiderVehicleGraphicsSet>
#[derive(Debug, Clone)]
pub struct SpiderVehicleGraphicsSet {
    base_animation: Option<RotatedAnimation>,
    shadow_base_animation: Option<RotatedAnimation>,
    animation: Option<RotatedAnimation>,
    shadow_animation: Option<RotatedAnimation>,
    base_render_layer: RenderLayer, // Default: "higher-object-under"
    render_layer: RenderLayer,      // Default: "wires-above"
    autopilot_destination_visualisation_render_layer: RenderLayer, // Default: "object"
    light: Option<LightDefinition>,
    eye_light: Option<LightDefinition>,
    autopilot_destination_on_map_visualisation: Option<Animation>,
    autopilot_destination_queue_on_map_visualisation: Option<Animation>,
    autopilot_destination_visualisation: Option<Animation>,
    autopilot_destination_queue_visualisation: Option<Animation>,
    autopilot_path_visualisation_line_width: f32, // Default: 0.125
    autopilot_path_visualisation_on_map_line_width: f32, // Default: 2.0
    light_positions: Vec<Vec<Factorio2DVector>>,
}

/// <https://wiki.factorio.com/Prototype/Wall#pictures>
#[derive(Debug, Clone)]
pub struct WallPictures {
    single: SpriteVariations,
    straight_vertical: SpriteVariations,
    straight_horizontal: SpriteVariations,
    corner_right_down: SpriteVariations,
    corner_left_down: SpriteVariations,
    t_up: SpriteVariations,
    ending_right: SpriteVariations,
    ending_left: SpriteVariations,
    filling: Option<SpriteVariations>,
    water_connection_patch: Option<Sprite4Way>,
    gate_connection_patch: Option<Sprite4Way>,
}
