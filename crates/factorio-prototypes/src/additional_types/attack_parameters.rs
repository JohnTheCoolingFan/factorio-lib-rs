use super::graphics::RotatedAnimation;
use super::sound::{CyclicSound, LayeredSound};
use super::{
    AmmoType, CircularParticleCreationSpecification, CircularProjectileCreationSpecification,
    Factorio2DVector, RangeMode,
};
use crate::util::defaults::*;
use serde::Deserialize;
use strum_macros::{AsRefStr, EnumString};

// ========== // AttackParameters // ========== //

/// <https://wiki.factorio.com/Types/AttackParameters>
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AttackParameters {
    // Depends on `type` key
    Projectile(ProjectileAttackParameters), // "projectile"
    Beam(BeamAttackParameters),             // "beam"
    Stream(StreamAttackParameters),         // "stream"
}

#[derive(Debug, Clone, Deserialize)]
pub struct BaseAttackParameters {
    #[serde(flatten)]
    pub range: AttackParametersRange,
    pub cooldown: AttackParametersCooldown,
    #[serde(default)]
    pub min_range: f32, // Default: 0
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub turn_range: f32, // Default: 1
    #[serde(default)]
    pub fire_penalty: f32, // Default: 0
    #[serde(default)]
    pub rotate_penalty: f32, // Default: 0
    #[serde(default)]
    pub health_penalty: f32, // Default: 0
    #[serde(default = "default_rangemode_center_to_center")]
    pub range_mode: RangeMode, // Default: "center-to-center"
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub damage_modifier: f32, // Default: 1
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub ammo_consumption_modifier: f32, // Default: 1
    #[serde(default)]
    pub cooldown_deviation: f32, // Default: 0
    #[serde(default)]
    pub warmup: u32, // Default: 0
    #[serde(default)]
    pub lead_target_for_projectile_speed: f32, // Default: 0
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub movement_slow_down_factor: f64, // Default: 1
    #[serde(default = "default_activation_type_shoot")]
    pub activation_type: ActivationType, // Default: "shoot"
    pub sound: Option<LayeredSound>,
    pub animation: Option<RotatedAnimation>,
    pub cyclic_sound: Option<CyclicSound>,
    #[serde(default)]
    pub use_shooter_direction: bool, // Default: false
    #[serde(flatten)]
    pub ammo_categories: AmmoCategories,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AttackParametersRange {
    Both {
        range: f32,
        min_attack_distance: f32,
    },
    Single {
        range: f32,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AttackParametersCooldown {
    Both {
        cooldown: f32,
        movement_slow_down_cooldown: f32,
    },
    Single {
        cooldown: f32,
    },
}

const fn default_rangemode_center_to_center() -> RangeMode {
    RangeMode::CenterToCenter
}

const fn default_activation_type_shoot() -> ActivationType {
    ActivationType::Shoot
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AmmoCategories {
    Many {
        ammo_type: AmmoType,
        ammo_categories: Vec<String>, // Names of AmmoCategory
    },
    Single {
        ammo_category: String,
    },
}

/// <https://wiki.factorio.com/Types/ProjectileAttackParameters>
#[derive(Debug, Clone, Deserialize)]
pub struct ProjectileAttackParameters {
    #[serde(flatten)]
    pub base: BaseAttackParameters,
    #[serde(default)]
    pub projectile_center: Factorio2DVector, // Default: (0, 0)
    #[serde(default)]
    pub projectile_creation_distance: f32, // Default: 0
    pub shell_particle: Option<CircularParticleCreationSpecification>,
    pub projectile_creation_parameters: Option<CircularProjectileCreationSpecification>,
    #[serde(default)]
    pub projectile_orientation_offset: f32, // Default: 0
}

/// <https://wiki.factorio.com/Types/BeamAttackParameters>
#[derive(Debug, Clone, Deserialize)]
pub struct BeamAttackParameters {
    #[serde(flatten)]
    pub base: BaseAttackParameters,
    #[serde(default)]
    pub source_direction_count: u32, // Default: 0
    pub source_offset: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters>
#[derive(Debug, Clone, Deserialize)]
pub struct StreamAttackParameters {
    #[serde(flatten)]
    base: BaseAttackParameters,
    #[serde(default)]
    pub fluid_consumption: f32, // Default: 0
    #[serde(default)]
    pub gun_barrel_length: f32, // Default: 0
    pub projectile_creation_parameters: Option<CircularProjectileCreationSpecification>,
    pub gun_center_shift: Option<GunCenterShift>,
    pub fluids: Vec<StreamAttackFluid>,
}

// =============== // Other // ================ //

/// <https://wiki.factorio.com/Types/BaseAttackParameters#activation_type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ActivationType {
    Shoot,
    Throw,
    Consume,
    Activate,
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters#gun_center_shift>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum GunCenterShift {
    Directions {
        north: Factorio2DVector,
        east: Factorio2DVector,
        south: Factorio2DVector,
        west: Factorio2DVector,
    },
    Single(Factorio2DVector),
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters#fluids>
#[derive(Debug, Clone, Deserialize)]
pub struct StreamAttackFluid {
    #[serde(rename = "type")]
    pub fluid: String, // Name of Fluid
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub damage_modifier: f64, // Default: 1
}
