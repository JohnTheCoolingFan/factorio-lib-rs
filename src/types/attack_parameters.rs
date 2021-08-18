use std::str::FromStr;
use std::fmt;
use crate::types::{Factorio2DVector, PrototypesErr, RangeMode, AmmoType, CircularParticleCreationSpecification};
use super::graphics::{RotatedAnimation};
use super::sound::{LayeredSound, CyclicSound};

// ========== // AttackParameters // ========== //

/// <https://wiki.factorio.com/Types/AttackParameters>
#[derive(Debug)]
pub enum AttackParameters {
    // Depends on `type` key
    Projectile(ProjectileAttackParameters), // "projectile"
    Beam(BeamAttackParameters), // "beam"
    Stream(StreamAttackParameters) // "stream"
}

/// <https://wiki.factorio.com/Types/ProjectileAttackParameters>
#[derive(Debug)]
pub struct ProjectileAttackParameters {
    range: f32,
    cooldown: f32,
    min_range: f32, // Default: 0
    turn_range: f32, // Default: 1
    fire_penalty: f32, // Default: 0
    rotate_penalty: f32, // Default: 0
    health_penalty: f32, // Default: 0
    range_mode: RangeMode, // Default: "center-to-center"
    min_attack_distance: f32, // Default: `range`
    damage_modifier: f32, // Default: 1
    ammo_consumption_modifier: f32, // Default: 1
    cooldown_deviation: f32, // Default: 0
    warmup: u32, // Default: 0
    lead_target_for_projectile_speed: f32, // Default: 0
    movement_slow_down_cooldown: f32, // Default: `cooldown`
    movement_slow_down_factor: f64, // Default: 1
    ammo_type: Option<AmmoType>,
    activation_type: Option<ActivationType>,
    sound: Option<LayeredSound>,
    animation: Option<RotatedAnimation>,
    cyclic_sound: Option<CyclicSound>,
    use_shooter_direction: bool, // Default: false
    ammo_categories: Option<Vec<String>>, // (Names) Name of AmmoCategory
    prohectile_center: Factorio2DVector, // Default: (0, 0)
    projectile_creation_distance: f32, // Default: 0
    shell_particle: Option<CircularParticleCreationSpecification>,
    projectile_creation_parameters: Option<CircularParticleCreationSpecification>,
    projectile_orientation_offset: f32, // Default: 0
}

/// <https://wiki.factorio.com/Types/BeamAttackParameters>
#[derive(Debug)]
pub struct BeamAttackParameters {
    range: f32,
    cooldown: f32,
    min_range: f32, // Default: 0
    turn_range: f32, // Default: 1
    fire_penalty: f32, // Default: 0
    rotate_penalty: f32, // Default: 0
    health_penalty: f32, // Default: 0
    range_mode: RangeMode, // Default: "center-to-center"
    min_attack_distance: f32, // Default: `range`
    damage_modifier: f32, // Default: 1
    ammo_consumption_modifier: f32, // Default: 1
    cooldown_deviation: f32, // Default: 0
    warmup: u32, // Default: 0
    lead_target_for_projectile_speed: f32, // Default: 0
    movement_slow_down_cooldown: f32, // Default: `cooldown`
    movement_slow_down_factor: f64, // Default: 1
    ammo_type: Option<AmmoType>,
    activation_type: Option<ActivationType>,
    sound: Option<LayeredSound>,
    animation: Option<RotatedAnimation>,
    cyclic_sound: Option<CyclicSound>,
    use_shooter_direction: bool, // Default: false
    ammo_categories: Option<Vec<String>>, // (Names) Name of AmmoCategory
    source_direction_count: u32, // Default: 0
    source_offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters>
#[derive(Debug)]
pub struct StreamAttackParameters {
    range: f32,
    cooldown: f32,
    min_range: f32, // Default: 0
    turn_range: f32, // Default: 1
    fire_penalty: f32, // Default: 0
    rotate_penalty: f32, // Default: 0
    health_penalty: f32, // Default: 0
    range_mode: RangeMode, // Default: "center-to-center"
    min_attack_distance: f32, // Default: `range`
    damage_modifier: f32, // Default: 1
    ammo_consumption_modifier: f32, // Default: 1
    cooldown_deviation: f32, // Default: 0
    warmup: u32, // Default: 0
    lead_target_for_projectile_speed: f32, // Default: 0
    movement_slow_down_cooldown: f32, // Default: `cooldown`
    movement_slow_down_factor: f64, // Default: 1
    ammo_type: Option<AmmoType>,
    activation_type: Option<ActivationType>,
    sound: Option<LayeredSound>,
    animation: Option<RotatedAnimation>,
    cyclic_sound: Option<CyclicSound>,
    use_shooter_direction: bool, // Default: false
    ammo_categories: Option<Vec<String>>, // (Names) Name of AmmoCategory
    fluid_consumption: f32, // Default: 0
    gun_barrel_length: f32, // Default: 0
    projectile_creation_parameters: Option<CircularParticleCreationSpecification>,
    gun_center_shift: Option<GunCenterShift>,
    fluids: Vec<StreamAttackFluid>
}

// =============== // Other // ================ //

/// <https://wiki.factorio.com/Types/BaseAttackParameters#activation_type>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ActivationType {
    Shoot,
    Throw,
    Consume,
    Activate,
}

impl FromStr for ActivationType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shoot" => Ok(Self::Shoot),
            "throw" => Ok(Self::Throw),
            "consume" => Ok(Self::Consume),
            "activate" => Ok(Self::Activate),
            _ => Err(PrototypesErr::InvalidTypeStr("ActivationType".into(), s.into()))
        }
    }
}

impl fmt::Display for ActivationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Shoot => "shoot",
            Self::Throw => "throw",
            Self::Consume => "consume",
            Self::Activate => "activate",
        })
    }
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters#gun_center_shift>
#[derive(Debug)]
pub struct GunCenterShift {
    north: Factorio2DVector,
    east: Factorio2DVector,
    south: Factorio2DVector,
    west: Factorio2DVector
}

/// <https://wiki.factorio.com/Types/StreamAttackParameters#fluids>
#[derive(Debug)]
pub struct StreamAttackFluid {
    r#type: String, // Name of Fluid
    damage_modifier: f64, // Default: 1
}
