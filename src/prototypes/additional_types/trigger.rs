use std::cmp;
use crate::prototypes::{GetPrototype, PrototypeFromLua, DataTable};
use super::{Factorio2DVector, CollisionMask, CollisionMode, EntityPrototypeFlags, ForceCondition, DamageTypeFilters, DamagePrototype, BoundingBox};
use super::sound::Sound;
use super::graphics::RenderLayer;
use factorio_lib_rs_derive::{TriggerItemBase, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase};
use strum::{EnumDiscriminants, EnumString};

// ============== // Trigger // =============== //

/// <https://wiki.factorio.com/Types/Trigger>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "kebab-case"))]
pub enum Trigger {
    Direct(DirectTriggerItem),
    Area(AreaTriggerItem),
    Line(LineTriggerItem),
    Cluster(ClusterTriggerItem)
}

impl<'lua> PrototypeFromLua<'lua> for Trigger {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = &value {
            Ok(match table.get::<_, String>("type")?.parse::<TriggerDiscriminants>().map_err(mlua::Error::external)? {
                TriggerDiscriminants::Direct => Trigger::Direct(DirectTriggerItem::prototype_from_lua(value, lua, data_table)?),
                TriggerDiscriminants::Area => Trigger::Area(AreaTriggerItem::prototype_from_lua(value, lua, data_table)?),
                TriggerDiscriminants::Line => Trigger::Line(LineTriggerItem::prototype_from_lua(value, lua, data_table)?),
                TriggerDiscriminants::Cluster => Trigger::Cluster(ClusterTriggerItem::prototype_from_lua(value, lua, data_table)?),
            })
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Trigger", message: Some("Expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/TriggerItem>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct TriggerItem {
    #[default(EntityPrototypeFlags::ALL)]
    pub entity_flags: EntityPrototypeFlags, // Default: all flags
    #[default(false)]
    pub ignore_collision_condition: bool, // Default: false
    #[default(TriggerTargetMask::Everything)]
    pub trigger_target_mask: TriggerTargetMask, // Default: all flags
    #[default(1_u32)]
    pub repeat_count: u32, // Default: 1
    #[default(1_f32)]
    pub probability: f32, // Default: 1
    #[default(CollisionMask::ALL)]
    pub collision_mask: CollisionMask, // Default: all
    pub action_delivery: Option<Vec<TriggerDelivery>>,
    #[default("all")]
    #[from_str]
    pub force: ForceCondition // Default: all forces
}

/// <https://wiki.factorio.com/Types/TriggerItem>
pub trait TriggerItemBase {
    fn entity_flags(&self) -> EntityPrototypeFlags; // Default: all flags
    fn ignore_collision_condition(&self) -> bool; // Default: false
    fn trigger_target_mask(&self) -> &TriggerTargetMask; // Default: all flags
    fn repeat_count(&self) -> u32; // Default: 1
    fn probability(&self) -> f32; // Default: 1
    fn collision_mask(&self) -> CollisionMask; // Default: all
    fn action_delivery(&self) -> &Option<Vec<TriggerDelivery>>;
    fn force(&self) -> ForceCondition; // Default: all forces
}

/// <https://wiki.factorio.com/Types/DirectTriggerItem>
#[derive(Debug, Clone, TriggerItemBase, PrototypeFromLua)]
pub struct DirectTriggerItem {
    #[use_self_forced]
    pub base: TriggerItem,
    #[default(false)]
    pub filter_enabled: bool // Default: false
}

/// <https://wiki.factorio.com/Types/AreaTriggerItem>
#[derive(Debug, Clone, TriggerItemBase, PrototypeFromLua)]
pub struct AreaTriggerItem {
    #[use_self_forced]
    pub base: TriggerItem,
    pub radius: f64,
    #[default(false)]
    pub trigger_from_target: bool, // Default: false
    #[default(true)]
    pub target_entities: bool, // Default: true
    #[default(true)]
    pub show_in_tooltip: bool, // Default: true
    #[from_str]
    #[default("distance-from-collision-box")]
    pub collision_mode: CollisionMode // Default: "distance-from-collision-box"
}

/// <https://wiki.factorio.com/Types/LineTriggerItem>
#[derive(Debug, Clone, TriggerItemBase, PrototypeFromLua)]
pub struct LineTriggerItem {
    #[use_self_forced]
    pub base: TriggerItem,
    pub range: f64,
    pub width: f64,
    pub range_effects: Option<TriggerEffect>
}

/// <https://wiki.factorio.com/Types/ClusterTriggerItem>
#[derive(Debug, Clone, TriggerItemBase, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct ClusterTriggerItem {
    #[use_self_forced]
    pub base: TriggerItem,
    pub cluster_count: f64, // Must be at least 2
    pub distance: f32,
    #[default(0_f32)]
    pub distance_deviation: f32 // Default: 0
}

impl ClusterTriggerItem {
    fn post_extr_fn(&self, _lua: &mlua::Lua, _data_table: &DataTable) -> mlua::prelude::LuaResult<()> {
        if self.cluster_count < 2.0 {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "ClusterTriggerItem", message: Some("`cluster_count` must be at least 2".into()) })
        }
        Ok(())
    }
}

// ========== // TriggerDelivery // =========== //

/// <https://wiki.factorio.com/Types/TriggerDelivery>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "kebab-case"))]
pub enum TriggerDelivery {
    Instant(InstantTriggerDelivery),
    Projectile(ProjectileTriggerDelivery),
    FlameThrower(FlameThrowerExplosionTriggerDelivery),
    Beam(BeamTriggerDelivery),
    Stream(StreamTriggerDelivery),
    Artillery(ArtilleryTriggerDelivery)
}

impl<'lua> PrototypeFromLua<'lua> for TriggerDelivery {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = &value {
            Ok(match table.get::<_, String>("type")?.parse::<TriggerDeliveryDiscriminants>().map_err(mlua::Error::external)? {
                TriggerDeliveryDiscriminants::Instant => TriggerDelivery::Instant(InstantTriggerDelivery::prototype_from_lua(value, lua, data_table)?),
                TriggerDeliveryDiscriminants::Projectile => TriggerDelivery::Projectile(ProjectileTriggerDelivery::prototype_from_lua(value, lua, data_table)?),
                TriggerDeliveryDiscriminants::FlameThrower => TriggerDelivery::FlameThrower(FlameThrowerExplosionTriggerDelivery::prototype_from_lua(value, lua, data_table)?),
                TriggerDeliveryDiscriminants::Beam => TriggerDelivery::Beam(BeamTriggerDelivery::prototype_from_lua(value, lua, data_table)?),
                TriggerDeliveryDiscriminants::Stream => TriggerDelivery::Stream(StreamTriggerDelivery::prototype_from_lua(value, lua, data_table)?),
                TriggerDeliveryDiscriminants::Artillery => TriggerDelivery::Artillery(ArtilleryTriggerDelivery::prototype_from_lua(value, lua, data_table)?),
            })
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TriggerDelivery", message: Some("Expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/InstantTriggerDelivery>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct InstantTriggerDelivery {
    pub source_effects: Option<TriggerEffect>,
    pub target_effects: Option<TriggerEffect>
}

/// <https://wiki.factorio.com/Types/ProjectileTriggerDelivery>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ProjectileTriggerDelivery {
    pub source_effects: Option<TriggerEffect>,
    pub target_effects: Option<TriggerEffect>,
    pub projectile: String,
    pub starting_speed: f32,
    #[default(0_f32)]
    pub starting_speed_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub direction_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub range_deviation: f32, // Default: 0
    #[default(1000_f64)]
    pub max_range: f64, // Default: 1000
    #[default(0_f64)]
    pub min_range: f64 // Default: 0
}

/// <https://wiki.factorio.com/Types/FlameThrowerExplosionTriggerDelivery>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct FlameThrowerExplosionTriggerDelivery {
    pub source_effects: Option<TriggerEffect>,
    pub target_effects: Option<TriggerEffect>,
    pub explosion: String,
    pub starting_distance: f64,
    #[default(0_f32)]
    pub direction_deviation: f32, // Default: 0
    #[default(0_f64)]
    pub speed_deviation: f64, // Default: 0
    #[default(0_f64)]
    pub starting_frame_fraction_deviation: f64, // Default: 0
    #[default(1_f64)]
    pub projectile_starting_speed: f64 // Default: 1
}

/// <https://wiki.factorio.com/Types/BeamTriggerDelivery>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct BeamTriggerDelivery {
    pub source_effects: Option<TriggerEffect>,
    pub target_effects: Option<TriggerEffect>,
    pub beam: String, // Name of Beam prototype
    #[default(true)]
    pub add_to_shooter: bool, // Default: true
    #[default(0_u32)]
    pub max_length: u32, // Default: 0
    #[default(0_u32)]
    pub duration: u32, // Default: 0
    pub source_offset: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/StreamTriggerDelivery>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct StreamTriggerDelivery {
    pub source_effects: Option<TriggerEffect>,
    pub target_effects: Option<TriggerEffect>,
    pub stream: String, // Name of FluidStream prototype
    pub source_offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/ArtilleryTriggerDelivery>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ArtilleryTriggerDelivery {
    pub source_effects: Option<TriggerEffect>,
    pub target_effects: Option<TriggerEffect>,
    pub projectile: String, // Name of ArtilleryProjectile prototype
    pub starting_speed: f32,
    #[default(0_f32)]
    pub starting_speed_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub direction_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub range_deviation: f32, // Default: 0
    #[default(false)]
    pub trigger_fired_artillery: bool // Default: false
}

// ============ // TriggerEffect // =========== //

/// <https://wiki.factorio.com/Types/TriggerEffect>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all="kebab-case"))]
pub enum TriggerEffect {
    Damage(DamageTriggerEffectItem),
    CreateEntity(CreateEntityTriggerEffectItem),
    CreateExplosion(CreateExplosionTriggerEffectItem),
    CreateFire(CreateFireTriggerEffectItem),
    CreateSmoke(CreateSmokeTriggerEffectItem),
    CreateTrivialSmoke(CreateTrivialSmokeEffectItem),
    CreateParticle(CreateParticleTriggerEffectItem),
    CreateSticker(CreateStickerTriggerEffectItem),
    CreateDecorative(CreateDecorativesTriggerEffectItem),
    NestedResult(Box<NestedTriggerEffectItem>),
    PlaySound(PlaySoundTriggerEffectItem),
    PushBack(PushBackTriggerEffectItem),
    DestroyCliffs(DestroyCliffsTriggerEffectItem),
    ShowExplosionOnChart(ShowExplosionOnChartTriggerEffectItem),
    InsertItem(InsertItemTriggerEffectItem),
    Script(ScriptTriggerEffectItem),
    SetTile(SetTileTriggerEffectItem),
    InvokeTileTrigger(InvokeTileEffectTriggerEffectItem),
    DestroyDecoratives(DestroyDecorativesTriggerEffectItem),
    CameraEffect(CameraEffectTriggerEffectItem),
}

impl<'lua> PrototypeFromLua<'lua> for TriggerEffect {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = &value {
            if let Some(te_type) = table.get::<_, Option<String>>("type")? {
                Ok(match te_type.parse::<TriggerEffectDiscriminants>().map_err(mlua::Error::external)? {
                    TriggerEffectDiscriminants::Damage => TriggerEffect::Damage(DamageTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateEntity => TriggerEffect::CreateEntity(CreateEntityTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateExplosion => TriggerEffect::CreateExplosion(CreateExplosionTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateFire => TriggerEffect::CreateFire(CreateFireTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateSmoke => TriggerEffect::CreateSmoke(CreateSmokeTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateTrivialSmoke => TriggerEffect::CreateTrivialSmoke(CreateTrivialSmokeEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateParticle => TriggerEffect::CreateParticle(CreateParticleTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateSticker => TriggerEffect::CreateSticker(CreateStickerTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CreateDecorative => TriggerEffect::CreateDecorative(CreateDecorativesTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::NestedResult => TriggerEffect::NestedResult(Box::new(NestedTriggerEffectItem::prototype_from_lua(value, lua, data_table)?)),
                    TriggerEffectDiscriminants::PlaySound => TriggerEffect::PlaySound(PlaySoundTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::PushBack => TriggerEffect::PushBack(PushBackTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::DestroyCliffs => TriggerEffect::DestroyCliffs(DestroyCliffsTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::ShowExplosionOnChart => TriggerEffect::ShowExplosionOnChart(ShowExplosionOnChartTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::InsertItem => TriggerEffect::InsertItem(InsertItemTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::Script => TriggerEffect::Script(ScriptTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::SetTile => TriggerEffect::SetTile(SetTileTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::InvokeTileTrigger => TriggerEffect::InvokeTileTrigger(InvokeTileEffectTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::DestroyDecoratives => TriggerEffect::DestroyDecoratives(DestroyDecorativesTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                    TriggerEffectDiscriminants::CameraEffect => TriggerEffect::CameraEffect(CameraEffectTriggerEffectItem::prototype_from_lua(value, lua, data_table)?),
                })
            } else {
                Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TriggerEffect", message: Some("Expected `type` field".into()) })
            }
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TriggerEffect", message: Some("Expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/TriggerEffectItem>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct TriggerEffectItem {
    #[default(1_u16)]
    pub repeat_count: u16, // Default: 1
    #[default(1_u16)]
    pub repeat_count_deviation: u16, // Default: 0
    #[default(1_f32)]
    pub probability: f32, // Default: 1 // 0 < value <= 1
    #[default(false)]
    pub affects_target: bool, // Default: false
    // We can safely use `.ok()?` because type was found to be a string earlier
    #[default({let te_type = prot_table.get::<_, String>("type").ok()?; !["create-entity", "create-explosion", "create-fire", "create-smoke", "create-particle", "create-sticker"].contains(&te_type.as_ref())})]
    pub show_in_tooltip: bool, // Default: true // Default: false in some cases
    pub damage_type_filters: Option<DamageTypeFilters>
}

/// <https://wiki.factorio.com/Types/TriggerEffectItem>
pub trait TriggerEffectItemBase {
    fn repeat_count(&self) -> u16;
    fn repeat_count_deviation(&self) -> u16;
    fn probability(&self) -> f32;
    fn affects_target(&self) -> bool;
    fn show_in_tooltip(&self) -> bool;
    fn damage_type_filters(&self) -> &Option<DamageTypeFilters>;
}

/// <https://wiki.factorio.com/Types/DamageTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct DamageTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub damage: DamagePrototype,
    #[default(true)]
    pub apply_damage_to_trees: bool, // Default: true
    #[default(false)]
    pub vaporize: bool, // Default: false
    #[default(u16::MAX)]
    pub lower_distance_threshold: u16, // Default: u16::MAX
    #[default(u16::MAX)]
    pub upper_distance_threshold: u16, // Default: u16::MAX
    #[default(1_f32)]
    pub lower_damage_modifier: f32, // Default: 1
    #[default(1_f32)]
    pub upper_damage_modifier: f32  // Default: 1
}

/// <https://wiki.factorio.com/Types/CreateEntityTriggerEffectItem>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct CreateEntityTriggerEffect {
    pub entity_name: String, // Entity name
    pub offset_deviation: Option<BoundingBox>,
    #[default(false)]
    pub trigger_created_entity: bool, // Default: false
    #[default(false)]
    pub check_buildability: bool, // Default: false
    // Override default in constructor
    //show_in_tooltip: bool, // Default: false
    pub tile_collision_mask: Option<CollisionMask>,
    pub offsets: Option<Vec<Factorio2DVector>>
}

/// <https://wiki.factorio.com/Types/CreateEntityTriggerEffectItem>
pub trait CreateEntityTriggerEffectItemBase {
    fn entity_name(&self) -> &String;
    fn offset_deviation(&self) -> &Option<BoundingBox>;
    fn trigger_created_entity(&self) -> bool;
    fn check_buildability(&self) -> bool;
    //fn show_in_tooltip(&self) -> bool;
    fn tile_collision_mask(&self) -> &Option<CollisionMask>;
    fn offsets(&self) -> &Option<Vec<Factorio2DVector>>;
}

/// <https://wiki.factorio.com/Types/CreateEntityTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase, PrototypeFromLua)]
pub struct CreateEntityTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    #[use_self_forced]
    pub create_entity_base: CreateEntityTriggerEffect
}

/// <https://wiki.factorio.com/Types/CreateExplosionTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase, PrototypeFromLua)]
pub struct CreateExplosionTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    #[use_self_forced]
    pub create_entity_base: CreateEntityTriggerEffect,
    #[default(-1_f32)]
    pub max_movement_distance: f32, // Default: -1
    #[default(0_f32)]
    pub max_movement_distance_deviation: f32, // Default: 0
    #[default(false)]
    pub inherit_movement_distance_from_projectile: bool, // Default: false
    #[default(false)]
    pub cycle_while_moving: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CreateFireTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase, PrototypeFromLua)]
pub struct CreateFireTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    #[use_self_forced]
    pub create_entity_base: CreateEntityTriggerEffect,
    #[default(u8::MAX)]
    pub initial_ground_flame_count: u8 // Default: u8::MAX
}

/// <https://wiki.factorio.com/Types/CreateSmokeTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase, PrototypeFromLua)]
pub struct CreateSmokeTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    #[use_self_forced]
    pub create_entity_base: CreateEntityTriggerEffect,
    #[default(0_f32)]
    pub initial_height: f32, // Default: 0
    pub speed: Option<Factorio2DVector>,
    #[default(0_f32)]
    pub speed_multiplier: f32, // Default: 0
    #[default(0_f32)]
    pub speed_multiplier_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame: f32, // Default: 0 // Why is it f32?
    #[default(0_f32)]
    pub starting_frame_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame_speed: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame_speed_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub speed_from_center: f32, // Default: 0
    #[default(0_f32)]
    pub speed_from_center_deviation: f32 // Default: 0
}

/// <https://wiki.factorio.com/Types/CreateTrivialSmokeEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct CreateTrivialSmokeEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub smoke_name: String, // Name of TrivialSmoke prototype
    pub offset_deviation: Option<BoundingBox>,
    pub offsets: Option<Vec<Factorio2DVector>>,
    #[default(0_f32)]
    pub initial_height: f32, // Default: 0
    #[default(0_f32)]
    pub max_radius: f32, // Default: 0
    #[default(Factorio2DVector(0_f32, 0_f32))]
    pub speed: Factorio2DVector, // Default: (0, 0)
    #[default(0_f32)]
    pub speed_multiplier: f32, // Default: 0
    #[default(0_f32)]
    pub speed_multiplier_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame_speed: f32, // Default: 0
    #[default(0_f32)]
    pub starting_frame_speed_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub speed_from_center: f32, // Default: 0
    #[default(0_f32)]
    pub speed_from_center_deviation: f32 // Default: 0
}

/// <https://wiki.factorio.com/Types/CreateParticleTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct CreateParticleTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub particle_name: String, // Name of Particle prototype
    pub initial_height: f32,
    pub offset_deviation: Option<BoundingBox>,
    // show_in_tooltip: Default: false // Override in constructor
    pub tile_collision_mask: Option<CollisionMask>,
    pub offsets: Option<Vec<Factorio2DVector>>,
    #[default(0_f32)]
    pub initial_height_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub initial_vertical_speed: f32, // Default: 0
    #[default(0_f32)]
    pub initial_vertical_speed_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub speed_from_center: f32, // Default: 0
    #[default(0_f32)]
    pub speed_from_center_deviation: f32, // Default: 0
    #[default(1_f32)]
    pub frame_speed: f32, // Default: 1
    #[default(0_f32)]
    pub frame_speed_deviation: f32, // Default: 0
    #[default(0_u8)]
    pub tail_length: u8, // Default: 0 // Silently capped to maximum fo 100
    #[default(0_u8)]
    pub tail_length_deviation: u8, // Default: 0
    #[default(1_f32)]
    pub tail_width: f32, // Default: 1
    #[default(false)]
    pub rotate_offsets: bool // Default: false
}

impl CreateParticleTriggerEffectItem {
    fn post_extr_fn(&mut self, _lua: &mlua::Lua, _data_table: &DataTable) -> mlua::prelude::LuaResult<()> {
        self.tail_length = cmp::min(self.tail_length, 100);
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/CreateStickerTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct CreateStickerTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub stricker: String, // Name of Sticker prototype
    // show_in_tooltip: Default: false // Override in constructor
    #[default(false)]
    pub trigger_created_entity: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CreateDecorativesTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct CreateDecorativesTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub decorative: String, // name of Decorative prototype
    pub spawn_max: u16,
    pub spawn_min_radius: f32,
    pub spawn_max_radius: f32, // Must be < 24
    #[default(0_u16)]
    pub spawn_min: u16, // Default: 0
    #[default(0.5_f32)]
    pub radius_curve: f32, // Default: 0.5
    #[default(false)]
    pub apply_projection: bool, // Default: false
    #[default(false)]
    pub spread_evenly: bool // Default: false
}

impl CreateDecorativesTriggerEffectItem {
    fn post_extr_fn(&self, _lua: &mlua::Lua, _data_table: &DataTable) -> mlua::prelude::LuaResult<()> {
        if self.spawn_max_radius >= 24.0 {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "CreateDecorativesTriggerEffectItem", message: Some("`spawn_max_radius` must be < 24.0".into()) })
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/NestedTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct NestedTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub action: Trigger
}

/// <https://wiki.factorio.com/Types/PlaySoundTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct PlaySoundTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub sound: Sound,
    // Negative values are silently clamped to 0
    #[default(0_f32)]
    pub min_distance: f32, // Default: 0
    #[default(1e21_f32)]
    pub max_distance: f32, // Default: 1e21
    #[default(1_f32)]
    pub volume_modifier: f32, // Default: 1
    #[default(1_f32)]
    pub audible_distance_modifier: f32, // Default: 1
    #[default(false)]
    pub play_on_target_position: bool // Default: false
}

impl PlaySoundTriggerEffectItem {
    fn post_extr_fn(&mut self, _lua: &mlua::Lua, _data_table: &DataTable) -> mlua::prelude::LuaResult<()> {
        if self.min_distance.is_sign_negative() { self.min_distance = 0.0};
        if self.max_distance.is_sign_negative() { self.max_distance = 0.0};
        if self.volume_modifier.is_sign_negative() { self.volume_modifier = 0.0};
        if self.audible_distance_modifier.is_sign_negative() { self.audible_distance_modifier = 0.0};
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/PushBackTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct PushBackTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub distance: f32
}

/// <https://wiki.factorio.com/Types/DestroyCliffsTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct DestroyCliffsTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub radius: f32,
    pub explosion: Option<String>, // Name of an entity
}

/// <https://wiki.factorio.com/Types/ShowExplosionOnChartTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub scale: f32
}

/// <https://wiki.factorio.com/Types/InsertItemTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct InsertItemTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub item: String, // Name of an item
    #[default(1_u32)]
    pub count: u32 // Default: 1
}

/// <https://wiki.factorio.com/Types/ScriptTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct ScriptTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub effect_id: String
}

/// <https://wiki.factorio.com/Types/SetTileTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct SetTileTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub tile_name: String, // Name of a prototype
    pub radius: f32,
    #[default(false)]
    pub apply_projection: bool, // Default: false
    #[default(CollisionMask(0))]
    pub tile_collision_mask: CollisionMask // Default: none
}

/// <https://wiki.factorio.com/Types/InvokeTileEffectTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct InvokeTileEffectTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub tile_collision_mask: Option<CollisionMask>
}

/// <https://wiki.factorio.com/Types/DestroyDecorativesTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct DestroyDecorativesTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub radius: f32,
    #[default("water-tile")]
    #[from_str]
    pub from_render_layer: RenderLayer, // Default: first layer
    #[default("cursor")]
    #[from_str]
    pub to_render_layer: RenderLayer, // Default: last layer
    #[default(false)]
    pub include_soft_decoratives: bool, // Default: false
    #[default(false)]
    pub include_decals: bool, // Default: false
    #[default(true)]
    pub invoke_decorative_trigger: bool, // Default: true
    #[default(false)]
    pub decoratives_with_trigger_only: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CameraEffectTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, PrototypeFromLua)]
pub struct CameraEffectTriggerEffectItem {
    #[use_self_forced]
    pub base: TriggerEffectItem,
    pub effect: String,
    pub duration: u8,
    #[default(0_u8)]
    pub ease_in_duration: u8, // Default: 0
    #[default(0_u8)]
    pub ease_out_duration: u8, // Default: 0
    #[default(0_u8)]
    pub delay: u8, // Default: 0
    #[default(0_u16)]
    pub full_strength_max_distance: u16, // Default: 0
    #[default(0_u16)]
    pub max_distance: u16, // Default: 0
    #[default(0_f32)]
    pub strength: f32, // Default: 0
}

// =============== // Other // ================ //

/// <https://wiki.factorio.com/Types/FootstepTriggerEffectList>
pub type FootstepTriggerEffectList = Vec<FootstepTriggerEffect>;

/// <https://wiki.factorio.com/Types/FootstepTriggerEffectList>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct FootstepTriggerEffect {
    #[use_self_vec]
    pub actions: Vec<CreateParticleTriggerEffectItem>,
    #[default(false)]
    pub use_as_default: bool, // Default: false
    pub tiles: Vec<String>, // (Names) Name of tile
}

/// <https://wiki.factorio.com/Types/TriggerTargetMask>
#[derive(Debug, Clone)]
pub enum TriggerTargetMask {
    Everything,
    Specific(Vec<String>)
}

impl<'lua> PrototypeFromLua<'lua> for TriggerTargetMask {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, _data_table: &mut DataTable) -> mlua::Result<Self> {
        Ok(Self::Specific(lua.unpack::<Vec<String>>(value)?))
    }
}
