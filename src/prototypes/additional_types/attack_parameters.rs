use crate::prototypes::{PrototypeFromLua, GetPrototype};
use super::{Factorio2DVector, RangeMode, AmmoType, CircularProjectileCreationSpecification, CircularParticleCreationSpecification};
use super::graphics::RotatedAnimation;
use super::sound::{LayeredSound, CyclicSound};
use mlua::FromLua;
use strum_macros::{EnumString, AsRefStr, EnumDiscriminants};

// ========== // AttackParameters // ========== //

/// <https://wiki.factorio.com/Types/AttackParameters>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all="kebab-case"))]
pub enum AttackParameters {
    // Depends on `type` key
    Projectile(ProjectileAttackParameters), // "projectile"
    Beam(BeamAttackParameters), // "beam"
    Stream(StreamAttackParameters) // "stream"
}

impl<'lua> PrototypeFromLua<'lua> for AttackParameters {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = &value {
            Ok(match table.get::<_, String>("type")?.parse().map_err(mlua::Error::external)? {
                AttackParametersDiscriminants::Projectile => AttackParameters::Projectile(ProjectileAttackParameters::prototype_from_lua(value, lua, data_table)?),
                AttackParametersDiscriminants::Beam => AttackParameters::Beam(BeamAttackParameters::prototype_from_lua(value, lua, data_table)?),
                AttackParametersDiscriminants::Stream => AttackParameters::Stream(StreamAttackParameters::prototype_from_lua(value, lua, data_table)?),
            })
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "AttackParameters", message: Some("expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/ProjectileAttackParameters>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ProjectileAttackParameters {
    pub range: f32,
    pub cooldown: f32,
    #[default(0_f32)]
    pub min_range: f32, // Default: 0
    #[default(1_f32)]
    pub turn_range: f32, // Default: 1
    #[default(0_f32)]
    pub fire_penalty: f32, // Default: 0
    #[default(0_f32)]
    pub rotate_penalty: f32, // Default: 0
    #[default(0_f32)]
    pub health_penalty: f32, // Default: 0
    #[from_str]
    #[default("center-to-center")]
    pub range_mode: RangeMode, // Default: "center-to-center"
    #[default(range)]
    pub min_attack_distance: f32, // Default: `range`
    #[default(1_f32)]
    pub damage_modifier: f32, // Default: 1
    #[default(1_f32)]
    pub ammo_consumption_modifier: f32, // Default: 1
    #[default(0_f32)]
    pub cooldown_deviation: f32, // Default: 0
    #[default(0_u32)]
    pub warmup: u32, // Default: 0
    #[default(0_f32)]
    pub lead_target_for_projectile_speed: f32, // Default: 0
    #[default(cooldown)]
    pub movement_slow_down_cooldown: f32, // Default: `cooldown`
    #[default(1_f64)]
    pub movement_slow_down_factor: f64, // Default: 1
    pub ammo_type: Option<AmmoType>,
    pub activation_type: Option<ActivationType>,
    pub sound: Option<LayeredSound>,
    pub animation: Option<RotatedAnimation>,
    pub cyclic_sound: Option<CyclicSound>,
    #[default(false)]
    pub use_shooter_direction: bool, // Default: false
    pub ammo_categories: Option<Vec<String>>, // (Names) Name of AmmoCategory
    #[default(Factorio2DVector(0.0, 0.0))]
    pub prohectile_center: Factorio2DVector, // Default: (0, 0)
    #[default(0_f32)]
    pub projectile_creation_distance: f32, // Default: 0
    pub shell_particle: Option<CircularParticleCreationSpecification>,
    pub projectile_creation_parameters: Option<CircularProjectileCreationSpecification>,
    #[default(0_f32)]
    pub projectile_orientation_offset: f32, // Default: 0
}

/// <https://wiki.factorio.com/Types/BeamAttackParameters>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct BeamAttackParameters {
    pub range: f32,
    pub cooldown: f32,
    #[default(0_f32)]
    pub min_range: f32, // Default: 0
    #[default(1_f32)]
    pub turn_range: f32, // Default: 1
    #[default(0_f32)]
    pub fire_penalty: f32, // Default: 0
    #[default(0_f32)]
    pub rotate_penalty: f32, // Default: 0
    #[default(0_f32)]
    pub health_penalty: f32, // Default: 0
    #[from_str]
    #[default("center-to-center")]
    pub range_mode: RangeMode, // Default: "center-to-center"
    #[default(range)]
    pub min_attack_distance: f32, // Default: `range`
    #[default(1_f32)]
    pub damage_modifier: f32, // Default: 1
    #[default(1_f32)]
    pub ammo_consumption_modifier: f32, // Default: 1
    #[default(0_f32)]
    pub cooldown_deviation: f32, // Default: 0
    #[default(0_u32)]
    pub warmup: u32, // Default: 0
    #[default(0_f32)]
    pub lead_target_for_projectile_speed: f32, // Default: 0
    #[default(cooldown)]
    pub movement_slow_down_cooldown: f32, // Default: `cooldown`
    #[default(1_f64)]
    pub movement_slow_down_factor: f64, // Default: 1
    pub ammo_type: Option<AmmoType>,
    pub activation_type: Option<ActivationType>,
    pub sound: Option<LayeredSound>,
    pub animation: Option<RotatedAnimation>,
    pub cyclic_sound: Option<CyclicSound>,
    #[default(false)]
    pub use_shooter_direction: bool, // Default: false
    pub ammo_categories: Option<Vec<String>>, // (Names) Name of AmmoCategory
    #[default(0_u32)]
    pub source_direction_count: u32, // Default: 0
    pub source_offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct StreamAttackParameters {
    pub range: f32,
    pub cooldown: f32,
    #[default(0_f32)]
    pub min_range: f32, // Default: 0
    #[default(1_f32)]
    pub turn_range: f32, // Default: 1
    #[default(0_f32)]
    pub fire_penalty: f32, // Default: 0
    #[default(0_f32)]
    pub rotate_penalty: f32, // Default: 0
    #[default(0_f32)]
    pub health_penalty: f32, // Default: 0
    #[from_str]
    #[default("center-to-center")]
    pub range_mode: RangeMode, // Default: "center-to-center"
    #[default(range)]
    pub min_attack_distance: f32, // Default: `range`
    #[default(1_f32)]
    pub damage_modifier: f32, // Default: 1
    #[default(1_f32)]
    pub ammo_consumption_modifier: f32, // Default: 1
    #[default(0_f32)]
    pub cooldown_deviation: f32, // Default: 0
    #[default(0_u32)]
    pub warmup: u32, // Default: 0
    #[default(0_f32)]
    pub lead_target_for_projectile_speed: f32, // Default: 0
    #[default(cooldown)]
    pub movement_slow_down_cooldown: f32, // Default: `cooldown`
    #[default(1_f64)]
    pub movement_slow_down_factor: f64, // Default: 1
    pub ammo_type: Option<AmmoType>,
    pub activation_type: Option<ActivationType>,
    pub sound: Option<LayeredSound>,
    pub animation: Option<RotatedAnimation>,
    pub cyclic_sound: Option<CyclicSound>,
    #[default(false)]
    pub use_shooter_direction: bool, // Default: false
    pub ammo_categories: Option<Vec<String>>, // (Names) Name of AmmoCategory
    #[default(0_f32)]
    pub fluid_consumption: f32, // Default: 0
    #[default(0_f32)]
    pub gun_barrel_length: f32, // Default: 0
    pub projectile_creation_parameters: Option<CircularProjectileCreationSpecification>,
    pub gun_center_shift: Option<GunCenterShift>,
    pub fluids: Vec<StreamAttackFluid>
}

// =============== // Other // ================ //

/// <https://wiki.factorio.com/Types/BaseAttackParameters#activation_type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ActivationType {
    Shoot,
    Throw,
    Consume,
    Activate,
}

impl<'lua> FromLua<'lua> for ActivationType {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        lua.unpack::<String>(value)?.parse().map_err(mlua::Error::external)
    }
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters#gun_center_shift>
#[derive(Debug, Clone)]
pub struct GunCenterShift {
    pub north: Factorio2DVector,
    pub east: Factorio2DVector,
    pub south: Factorio2DVector,
    pub west: Factorio2DVector
}

impl<'lua> PrototypeFromLua<'lua> for GunCenterShift {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, _data_table: &mut crate::prototypes::DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(t) = &value {
            if let Some(north) = t.get::<_, Option<Factorio2DVector>>("north")? {
                let east = t.get::<_, Factorio2DVector>("east")?;
                let south = t.get::<_, Factorio2DVector>("south")?;
                let west = t.get::<_, Factorio2DVector>("west")?;
                Ok(Self{north, east, south, west})
            } else {
                let vector = lua.unpack::<Factorio2DVector>(value)?;
                Ok(Self{north: vector, east: vector, south: vector, west: vector})
            }
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "GunCenterShift", message: Some("Expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters#fluids>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct StreamAttackFluid {
    #[rename("type")]
    pub fluid: String, // Name of Fluid
    #[default(1_f64)]
    pub damage_modifier: f64, // Default: 1
}
