use super::{GetPrototype, PrototypeFromLua, DataTable};
use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign};
use std::iter::{Iterator, FromIterator};
use super::{Factorio2DVector, Color, FileName, BoundingBox, RealOrientation, CreateParticleTriggerEffectItem};
use strum_macros::{EnumString, AsRefStr};
use mlua::{prelude::*, Value};

// ============ // Simple types // ============ //

/// List of 1-based frame indices into the spreadsheet
/// <https://wiki.factorio.com/Types/AnimationFrameSequence>
pub type AnimationFrameSequence = Vec<u16>;
/// <https://wiki.factorio.com/Types/Sprite#position>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpritePosition(pub i16, pub i16);
/// Width and Height <https://wiki.factorio.com/Types/Sprite#width>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpriteSize(pub i16, pub i16);
/// <https://wiki.factorio.com/Types/SpriteSizeType>
pub type SpriteSizeType = i16;

// =========== // General types // ============ //
        // Enums with FromStr

/// <https://wiki.factorio.com/Types/WorkingVisualisation#apply_recipe_tint>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ApplyRecipeTint {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    None,
}

/// <https://wiki.factorio.com/Types/WorkingVisualisation#apply_tint>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ApplyTint {
    ResourceColor,
    InputFluidBaseColor,
    InputFluidFlowColor,
    Status,
    None,
}


/// <https://wiki.factorio.com/Types/BeaconGraphicsSet#apply_module_tint>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ApplyModuleTint {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet#module_tint_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ModuleTintMode {
    SingleModule,
    Mix,
}

/// <https://wiki.factorio.com/Types/LightDefinition#type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum LightDefinitionType {
    Basic,
    Oriented,
}

/// <https://wiki.factorio.com/Types/BaseAttackParameters#range_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RangeMode {
    CenterToCenter,
    BoundingBoxToBoundingBox,
}

/// <https://wiki.factorio.com/Prototype/Lamp#glow_render_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum GlowRenderMode {
    Additive,
    Multiplicative,
}

/// <https://wiki.factorio.com/Types/RenderLayer>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
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

/// <https://wiki.factorio.com/Types/Sprite#draw_as_shadow>
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
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
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum BlendMode {
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    Overwrite
}

/// <https://wiki.factorio.com/Types/Animation#run_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RunMode {
    Forward,
    Backward,
    ForwardThenBackward
}

        // Structs

/// <https://wiki.factorio.com/Types/FluidBox#secondary_draw_orders>
#[derive(Debug, Clone, Eq, PartialEq, Copy, PrototypeFromLua)]
pub struct SecondaryDrawOrders {
    #[default(1)]
    pub north: i8,
    #[default(1)]
    pub east: i8,
    #[default(1)]
    pub south: i8,
    #[default(1)]
    pub west: i8
}

/// <https://wiki.factorio.com/Types/LightDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct LightDefinition {
    #[rename("type")]
    #[from_str]
    pub light_def_type: LightDefinitionType,
    // Next 2 are mandatory if type is "oriented"
    #[mandatory_if(light_def_type == LightDefinitionType::Oriented)]
    pub picture: Option<Sprite>,
    #[mandatory_if(light_def_type == LightDefinitionType::Oriented)]
    pub rotation_shift: Option<f32>,
    pub intensity: f32,
    pub size: f32,
    #[default(0.0_f32)]
    pub source_orientation_offset: f32, // Default: 0
    #[default(false)]
    pub add_perspective: bool, // Default: false
    pub shift: Option<Factorio2DVector>,
    #[default(Color(0.0, 0.0, 0.0, 0.0))]
    pub color: Color, // Default: no color
    #[default(0.0_f32)]
    pub minimum_darkness: f32 // Default: 0
}

/// <https://wiki.factorio.com/Types/LightFlickeringDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct LightFlickeringDefinition {
    #[default(0.2)]
    pub minimum_intensity: f32, // Default: 0.2
    #[default(0.8)]
    pub maximum_intensity: f32, // Default: 0.8
    #[default(0.3)]
    pub derivation_change_frequency: f32, // Default: 0.3
    #[default(0.06)]
    pub derivation_change_deviation: f32, // Default: 0.06
    #[default(0.02)]
    pub border_fix_speed: f32, // Default: 0.02
    #[default(0.5)]
    pub minimum_light_size: f32, // Default: 0.5
    #[default(0.5)]
    pub light_intensity_to_size_coefficient: f32, // Default: 0.5
    #[default(Color::new_rgb(1.0, 1.0, 1.0))]
    pub color: Color // Default: (1, 1, 1) (White)
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#default_recipe_tint>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RecipeTint {
    // All default to (1, 1, 1, 1), except special cases
    #[default(Color(1.0, 1.0, 1.0, 1.0))]
    pub primary: Color,
    #[default(Color(1.0, 1.0, 1.0, 1.0))]
    pub secondary: Color,
    #[default(Color(1.0, 1.0, 1.0, 1.0))]
    pub tertiary: Color,
    #[default(Color(1.0, 1.0, 1.0, 1.0))]
    pub quaternary: Color
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine#shift_animation_waypoints>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ShiftAnimationWaypoints {
    pub north: Option<Vec<Factorio2DVector>>,
    pub east: Option<Vec<Factorio2DVector>>,
    pub south: Option<Vec<Factorio2DVector>>,
    pub west: Option<Vec<Factorio2DVector>>
}

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
#[derive(Debug, Clone)]
pub struct CircuitConnectorSecondaryDrawOrder {
    pub north: i8,
    pub east: i8,
    pub south: i8,
    pub west: i8
}

impl CircuitConnectorSecondaryDrawOrder {
    pub fn new(draw_order: i8) -> Self {
        Self{north: draw_order, east: draw_order, south: draw_order, west: draw_order}
    }
}

impl<'lua> PrototypeFromLua<'lua> for CircuitConnectorSecondaryDrawOrder {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, _data_table: &mut DataTable) -> mlua::Result<Self> {
        if let Ok(num) = lua.unpack::<Option<i8>>(value.clone()) {
            Ok(Self::new(num.unwrap_or(100)))
        } else if let mlua::Value::Table(table) = &value {
            let north = table.get::<_, Option<i8>>("north")?.unwrap_or(100);
            let east = table.get::<_, Option<i8>>("east")?.unwrap_or(100);
            let south = table.get::<_, Option<i8>>("south")?.unwrap_or(100);
            let west = table.get::<_, Option<i8>>("west")?.unwrap_or(100);
            Ok(Self{north, east, south, west})
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "MiningDrillGraphicsSet.circuit_connector_secondary_draw_order",
            message: Some("Expected integer or table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Prototype/Entity#radius_visualisation_specification>
/// <https://wiki.factorio.com/Types/RadiusVisualisationSpecification>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct RadiusVisualizationSpecification {
    pub sprite: Option<Sprite>,
    #[default(0)]
    pub distance: f64, // Default: 0 // Must be > 0
    pub offset: Option<Factorio2DVector>,
    #[default(true)]
    pub draw_in_cursor: bool, // Default: true
    #[default(true)]
    pub draw_on_selection: bool // Default: true
}

impl RadiusVisualizationSpecification {
    fn post_extr_fn(&mut self, _lua: &mlua::Lua, _data_table: &DataTable) -> mlua::prelude::LuaResult<()> {
        if self.distance < 0.0 { // Not same as docs but makes sense
            // Error message says same as docs
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "RadiusVisualizationSpecification", message: Some("`distance` must be > 0".into()) })
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/WaterReflectionDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct WaterReflectionDefinition {
    pictures: Option<Vec<SpriteVariation>>,
    #[default(false)]
    orientation_to_variation: bool, // default: false
    #[default(false)]
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

// ============= // Animations // ============= //

/// <https://wiki.factorio.com/Types/Animation#layers>
#[derive(Debug, Clone)]
pub enum Animation {
    Layers(Vec<Animation>),
    Single(Box<AnimationBase>)
}

impl Animation {
    fn check_stripes(&self) -> bool {
        match self {
            Self::Layers(layers) => {
                let mut flag = false;
                for layer in layers {
                    flag |= layer.check_stripes()
                };
                flag
            },
            Self::Single(ab) => {
                ab.check_stripes()
            }
        }
    }
}

impl<'lua> PrototypeFromLua<'lua> for Animation {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut DataTable) -> mlua::Result<Self> {
        let type_name = &value.type_name();
        if let mlua::Value::Table(p_table) = value {
            let layers = p_table.get::<_, Option<Vec<mlua::Value>>>("layers")?;
            let result = if let Some(actual_layers) = layers {
                Self::Layers(actual_layers.into_iter().map(|v| Self::prototype_from_lua(v, lua, data_table)).collect::<mlua::Result<Vec<Self>>>()?)
            } else {
                Self::Single(Box::new(AnimationBase::prototype_from_lua(p_table.to_lua(lua)?, lua, data_table)?))
            };
            if result.check_stripes() {
                return Err(mlua::Error::FromLuaConversionError { from: type_name, to: "Animation",
                message: Some("`height_in_frames` in stripes is mandatory".into()) })
            };
            Ok(result)
        } else {
            Err(mlua::Error::FromLuaConversionError{from:type_name, to: "Animation", message: Some("Expected table".into())})
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
        self.regular.check_stripes() || { if let Some(ans) = &self.hr_version { ans.check_stripes() } else { false } }
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
    #[from_str]
    #[default("forward")]
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
    pub stripes: Option<Vec<Stripe>>
}

impl AnimationSpec {
    // TODO: clarify the required image sizes for stripes
    fn register_resources(&self, _lua: &mlua::Lua, _data_table: &mut DataTable) -> mlua::prelude::LuaResult<()> {
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
    pub y: u32 // Default: 0
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedAnimation {
    #[use_self_forced]
    pub regular: RotatedAnimationSpec,
    pub hr_version: Option<RotatedAnimationSpec>
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
    pub animation: AnimationVariation
}

/// <https://wiki.factorio.com/Types/RotatedAnimationVariations>
pub type RotatedAnimationVariations = Vec<RotatedAnimationVariation>;

/// <https://wiki.factorio.com/Types/RotatedAnimationVariations>
#[derive(Debug, Clone)]
pub enum RotatedAnimationVariation {
    Layers(Vec<RotatedAnimationVariation>),
    Single(Box<RotatedAnimation>)
}

impl<'lua> PrototypeFromLua<'lua> for RotatedAnimationVariation {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut DataTable) -> mlua::Result<Self> {
        let type_name = value.type_name();
        if let Value::Table(t) = value {
            if t.get::<_, Option<String>>("direction_count")?.is_some() {
                Ok(Self::Single(Box::new(RotatedAnimation::prototype_from_lua(t.to_lua(lua)?, lua, data_table)?)))
            } else {
                Ok(Self::Layers(t
                        .sequence_values::<Value>()
                        .map(|v| RotatedAnimationVariation::prototype_from_lua(v?, lua, data_table))
                        .collect::<LuaResult<Vec<RotatedAnimationVariation>>>()?))
            }
        } else {
            Err(LuaError::FromLuaConversionError { from: type_name, to: "RotatedAnimationVariation", message: Some("expected table".into()) })
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
    west: RotatedAnimation
}

/// <https://wiki.factorio.com/Prototype/Beam#light_animations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct LightAnimations {
    pub start: Option<Animation>,
    pub ending: Option<Animation>,
    pub head: Option<Animation>,
    pub tail: Option<Animation>,
    pub body: Option<Vec<AnimationVariation>>
}

// ============== // Sprites // ==============  //

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct Sprite {
    #[use_self_vec]
    pub layers: Vec<SpriteLayer>
}

/// <https://wiki.factorio.com/Types/Sprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteLayer {
    #[use_self_forced]
    pub regular: SpriteSpec,
    pub hr_version: Option<SpriteSpec>
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
    pub dice: Option<Dice>, // AKA slice // _y and _x are converted into this
    pub priority: SpritePriority, // Default: "medium"
    pub flags: Option<SpriteFlags>,
    pub size: SpriteSize,
    // Automatically converted to size
    // width
    // height
    pub position: Option<SpritePosition>,
    // Automatically converted to position
    // x
    // y
    pub shift: Factorio2DVector, // (0, 0) by default
    pub scale: f64, // 1 by default,
    pub draw_as: DrawAs, // all false by default
    pub mipmap_count: u8, // Default: 0
    pub apply_runtime_tint: bool, // Default: false
    pub tint: Color, // Default: (1, 1, 1, 1) (white)
    pub blend_mode: BlendMode, // Default: "normal"
    pub load_in_minimal_mode: bool, //Default: false
    pub premul_alpha: bool, // Default: true
    pub generate_sfd: bool // Default: false // Only used by sprites in UtilitySprites with "icon" flag
}

// TODO: do this with a macro

impl<'lua> PrototypeFromLua<'lua> for SpriteSpecWithoutFilename {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, _data_table: &mut DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(p_table) = value {
            let dice: Option<Dice> = {
                let dice_gen_opt: Option<i16> = p_table.get::<_, Option<i16>>("dice")?.or_else(|| p_table.get("slice").ok());
                if let Some(dice_gen) = dice_gen_opt {
                    Some(Dice::new(dice_gen))
                } else {
                    let x: Option<i16> = p_table.get::<_, Option<i16>>("dice_x")?.or_else(|| p_table.get("slice_x").ok());
                    let y: Option<i16> = p_table.get::<_, Option<i16>>("dice_y")?.or_else(|| p_table.get("slice_y").ok());
                    if let (Some(ax), Some(ay)) = (x, y) {
                        Some(Dice(ax, ay))
                    } else {
                        None
                    }
                }
            };
            let priority: SpritePriority = p_table.get::<_, Option<String>>("priority")?.unwrap_or_else(|| "medium".into()).parse().map_err(
                |_| mlua::Error::FromLuaConversionError{from: "String", to: "SpritePriority", message: Some("invalid value".into())}
                )?;
            let flags = p_table.get::<_, Option<Vec<String>>>("flags")?.map(SpriteFlags::from_iter);
            let size = {
                let error = Err(mlua::Error::FromLuaConversionError{from: "integer", to: "SpriteSizeType", message: Some("value must be in range 0-8192".into())});
                if let Some(s_value) = p_table.get::<_, Option<mlua::Value>>("size")? {
                    match lua.unpack::<i16>(s_value.clone()) {
                        Ok(size) => {
                            if (0..=8192_i16).contains(&size) {
                                SpriteSize(size, size)
                            } else {
                                return error
                            }
                        },
                        _ => {
                            let size = lua.unpack::<[i16; 2]>(s_value)?;
                            if (0..=8192_i16).contains(&size[0]) && (0..=8192_i16).contains(&size[1]) {
                                SpriteSize(size[0], size[1])
                            } else {
                                return error
                            }
                        }
                    }
                } else {
                    let width: SpriteSizeType = p_table.get("width")?;
                    let height: SpriteSizeType = p_table.get("height")?;
                    if (0..=8192_i16).contains(&width) && (0..=8192_i16).contains(&height) {
                        SpriteSize(width, height)
                    } else {
                        return error
                    }
                }
            };
            let position = {
                let x = p_table.get::<_, Option<SpriteSizeType>>("x")?.unwrap_or(0);
                let y = p_table.get::<_, Option<SpriteSizeType>>("y")?.unwrap_or(0);
                if x != 0 || y != 0 {
                    Some(SpritePosition(x, y))
                } else {
                    p_table.get::<_, Option<[SpriteSizeType; 2]>>("position")?.map(|pos| SpritePosition(pos[0], pos[1]))
                }
            };
            let shift = p_table.get::<_, Option<Factorio2DVector>>("shift")?.unwrap_or(Factorio2DVector(0.0, 0.0));
            let scale = p_table.get::<_, Option<f64>>("scale")?.unwrap_or(1.0);
            let draw_as = {
                let draw_as_shadow = p_table.get::<_, Option<bool>>("draw_as_shadow")?.unwrap_or(false);
                let draw_as_glow = p_table.get::<_, Option<bool>>("draw_as_glow")?.unwrap_or(false);
                let draw_as_light = p_table.get::<_, Option<bool>>("draw_as_light")?.unwrap_or(false);
                DrawAs::new(draw_as_shadow, draw_as_glow, draw_as_light)
            };
            let mipmap_count = p_table.get::<_, Option<u8>>("mipmap_count")?.unwrap_or(0);
            let apply_runtime_tint = p_table.get::<_, Option<bool>>("apply_runtime_tint")?.unwrap_or(false);
            let tint = p_table.get::<_, Option<Color>>("tint")?.unwrap_or(Color(1.0, 1.0, 1.0, 1.0));
            let blend_mode: BlendMode = p_table.get::<_, Option<String>>("blend_mode")?.unwrap_or_else(|| "normal".into()).parse()
                .map_err(|_| mlua::Error::FromLuaConversionError{from: "string", to: "BlendMode", message: Some("Invalid variant".into())})?;
            let load_in_minimal_mode = p_table.get::<_, Option<bool>>("load_in_minimal_mode")?.unwrap_or(false);
            let premul_alpha = p_table.get::<_, Option<bool>>("premul_alpha")?.unwrap_or(true);
            let generate_sfd = p_table.get::<_, Option<bool>>("generate_sfd")?.unwrap_or(false);
            Ok(Self{
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
                generate_sfd
            })
        } else {
            Err(mlua::Error::FromLuaConversionError{from: value.type_name(), to: "SpriteSpec", message: Some("Expected table".into())})
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
    fn new<'lua>(value: mlua::Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable, frames: u32) -> LuaResult<Self> {
        if let mlua::Value::Table(t) = &value {
            let frames = t.get::<_, Option<u32>>("frames")?.unwrap_or(frames);
            let sprite = SpriteSpec::prototype_from_lua(value, lua, data_table)?;
            Ok(Self{sprite, frames})
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "SpriteNWaySheet", message: Some("Expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/Sprite4Way>
#[derive(Debug, Clone)]
pub struct Sprite4Way(pub DirectionalSprite);

impl<'lua> PrototypeFromLua<'lua> for Sprite4Way {
    fn prototype_from_lua(value: LuaValue<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        let type_name = value.type_name();
        if let LuaValue::Table(t) = &value {
            if let Some(sheets) = t.get::<_, Option<Vec<Value>>>("sheets")? {
                let sheets = sheets.into_iter().map(|v| SpriteNWaySheet::new(v, lua, data_table, 4)).collect::<LuaResult<Vec<SpriteNWaySheet>>>()?;
                Ok(Self(sheets.into()))
            } else if let Some(sheet) = t.get::<_, Option<Value>>("sheet")? {
                let sheets = vec![SpriteNWaySheet::new(sheet, lua, data_table, 4)?];
                Ok(Self(sheets.into()))
            } else {
                Ok(Self(DirectionalSprite::Directions(SpriteDirections::prototype_from_lua(value, lua, data_table)?)))
            }
        } else {
            Err(LuaError::FromLuaConversionError { from: type_name, to: "Sprite8Way", message: Some("expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/Sprite8Way>
#[derive(Debug, Clone)]
pub struct Sprite8Way(pub DirectionalSprite);

impl<'lua> PrototypeFromLua<'lua> for Sprite8Way {
    fn prototype_from_lua(value: LuaValue<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        let type_name = value.type_name();
        if let LuaValue::Table(t) = &value {
            if let Some(sheets) = t.get::<_, Option<Vec<Value>>>("sheets")? {
                let sheets = sheets.into_iter().map(|v| SpriteNWaySheet::new(v, lua, data_table, 8)).collect::<LuaResult<Vec<SpriteNWaySheet>>>()?;
                Ok(Self(sheets.into()))
            } else if let Some(sheet) = t.get::<_, Option<Value>>("sheet")? {
                let sheets = vec![SpriteNWaySheet::new(sheet, lua, data_table, 8)?];
                Ok(Self(sheets.into()))
            } else {
                Ok(Self(DirectionalSprite::Directions(SpriteDirections::prototype_from_lua(value, lua, data_table)?)))
            }
        } else {
            Err(LuaError::FromLuaConversionError { from: type_name, to: "Sprite8Way", message: Some("expected table".into()) })
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirectionalSprite {
    Sheets(Vec<SpriteNWaySheet>),
    Directions(SpriteDirections)
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
    pub layers: Vec<RotatedSpriteLayer>
}

/// <https://wiki.factorio.com/Types/RotatedSprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedSpriteLayer {
    #[use_self_forced]
    pub regular: RotatedSpriteSpec,
    pub hr_version: Option<RotatedSpriteSpec>
}

/// <https://wiki.factorio.com/Types/RotatedSprite>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RotatedSpriteSpec {
    pub sprites: Vec<SpriteSpec>, // If `filenames` is set, copy all properties to each object for each filename // FIXME
    pub direction_count: u16
}

/// <https://wiki.factorio.com/Types/SpriteVariations>
pub type SpriteVariations = Vec<SpriteVariation>;

/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteVariation {
    #[use_self_vec]
    pub layers: Vec<SpriteVariationLayer>
}

/// <https://wiki.factorio.com/Types/SpriteVariations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpriteVariationLayer {
    #[use_self_forced]
    pub regular: SpriteVariationSpec,
    pub hr_version: Option<SpriteVariationSpec>
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
    pub line_length: u32 // Default: value of `variation_count`
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
    NoAtlas
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
    pub rail_endings: Sprite8Way
}

/// <https://wiki.factorio.com/Types/RailPieceLayers>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct RailPieceLayers {
    // TODO: checks
    pub metals: Vec<SpriteVariation>,
    pub backplayes: Vec<SpriteVariation>, // Must have same number of variations as `metals`
    pub ties: Vec<SpriteVariation>, // Must have between 1 and 4 items
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
    pub red_green_led_light_offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/BeaconGraphicsSet>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct BeaconModuleVisualizations {
    art_style: String,
    use_for_empty_slots: bool, // Default: false
    tier_offset: i32, // Default: 0
    slots: Option<Vec<Vec<BeaconModuleVisualization>>>
}

/// <https://wiki.factorio.com/Types/BeaconModuleVisualization>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum WorkingVisualisationEffect {
    Flicker,
    UraniumGlow,
    None,
}

/// <https://wiki.factorio.com/Types/ConnectableEntityGraphics>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
    gas_flow: Animation

}

/// <https://wiki.factorio.com/Prototype/PipeToGround#pictures>
#[derive(Debug, Clone)]
pub struct PipeToGroundPictures {
    down: Sprite,
    up: Sprite,
    left: Sprite,
    ritgh: Sprite
}

/// <https://wiki.factorio.com/Prototype/Pump#fluid_wagon_connector_graphics>
#[derive(Debug, Clone)]
pub struct PumpConnectorGraphicsFluidWagon {
    load_animations: PumpConnectorGraphics,
    unload_animations: PumpConnectorGraphics
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
    Animations(AnimationVariations)
}

/// <https://wiki.factorio.com/Prototype/SimpleEntityWithOwner#pictures>
#[derive(Debug, Clone)]
pub enum SimpleEntityWithOwnerVisuals {
    Pictires(SpriteVariations),
    Pictire(Sprite4Way),
    Animations(AnimationVariations)
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
    lower_part_water_reflection: Option<SpiderLegPart>
}

/// <https://wiki.factorio.com/Types/SpiderLegPart>
#[derive(Debug, Clone)]
pub struct SpiderLegPart {
    top_end: Option<Sprite>,
    middle: Option<Sprite>,
    bottom_end: Option<Sprite>,
    middle_offset_from_top: f32, // Default: 0
    middle_offset_from_bottom: f32, // Default: 0
    top_end_length: f32, // Default: 0
    bottom_end_length: f32 // Default: 0
}

/// <https://wiki.factorio.com/Prototype/StorageTank#pictures>
#[derive(Debug, Clone)]
pub struct StorageTankPictures {
    picture: Sprite4Way,
    window_background: Sprite,
    fluid_background: Sprite,
    flow_sprite: Sprite,
    gas_flow: Animation
}

/// <https://wiki.factorio.com/Prototype/TrainStop#light1>
#[derive(Debug, Clone)]
pub struct TrainStopLight {
    sprite: Sprite4Way,
    red_picture: Sprite4Way,
    light: LightDefinition
}

/// <https://wiki.factorio.com/Prototype/TrainStop#drawing_boxes>
#[derive(Debug, Clone)]
pub struct TrainStopDrawingBoxes {
    north: BoundingBox,
    east: BoundingBox,
    south: BoundingBox,
    west: BoundingBox
}

/// <https://wiki.factorio.com/Prototype/TransportBeltConnectable#belt_animation_set>
#[derive(Debug, Clone)]
pub struct BeltAnimationSet {
    animation_set: RotatedAnimation,
    east_index: u8, // Default: 1
    west_index: u8, // Default: 2
    north_index: u8, // Default: 3
    south_index: u8, // Default: 4
    starting_south_index: u8, // Default: 13
    ending_south_index: u8, // Default: 14
    starting_west_index: u8, // Default: 15
    ending_west_index: u8, // Default: 16
    starting_north_index: u8, // Default: 17
    ending_north_index: u8, // Default: 18
    starting_east_index: u8, // Default: 19
    ending_east_index: u8, // Default: 20
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
    front_patch: Option<Sprite4Way>
}

/// <https://wiki.factorio.com/Prototype/LinkedBelt#structure>
#[derive(Debug, Clone)]
pub struct BeltStructureWithSideLoading {
    base_structure: BeltStructure,
    direction_in_side_loading: Option<Sprite4Way>,
    direction_out_side_loading: Option<Sprite4Way>
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
    frame_front_patch: Option<SpriteVariations>
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
    Variations(Vec<TreePrototypeVariation>) // Non-empty array
}

/// <https://wiki.factorio.com/Prototype/Tree#pictures>
#[derive(Debug, Clone)]
pub struct TreePictures {
    pictures: SpriteVariations,
    color: Vec<Color>
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
    water_reflection: Option<WaterReflectionDefinition>
}

/// <https://wiki.factorio.com/Types/SpiderVehicleGraphicsSet>
#[derive(Debug, Clone)]
pub struct SpiderVehicleGraphicsSet {
    base_animation: Option<RotatedAnimation>,
    shadow_base_animation: Option<RotatedAnimation>,
    animation: Option<RotatedAnimation>,
    shadow_animation: Option<RotatedAnimation>,
    base_render_layer: RenderLayer, // Default: "higher-object-under"
    render_layer: RenderLayer, // Default: "wires-above"
    autopilot_destination_visualisation_render_layer: RenderLayer, // Default: "object"
    light: Option<LightDefinition>,
    eye_light: Option<LightDefinition>,
    autopilot_destination_on_map_visualisation: Option<Animation>,
    autopilot_destination_queue_on_map_visualisation: Option<Animation>,
    autopilot_destination_visualisation: Option<Animation>,
    autopilot_destination_queue_visualisation: Option<Animation>,
    autopilot_path_visualisation_line_width: f32, // Default: 0.125
    autopilot_path_visualisation_on_map_line_width: f32, // Default: 2.0
    light_positions: Vec<Vec<Factorio2DVector>>
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
    gate_connection_patch: Option<Sprite4Way>
}
