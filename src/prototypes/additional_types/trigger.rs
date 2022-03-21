use super::{Factorio2DVector, CollisionMask, CollisionMode, EntityPrototypeFlags, ForceCondition, DamageTypeFilters, DamagePrototype, BoundingBox};
use super::sound::Sound;
use super::graphics::RenderLayer;
use factorio_lib_rs_derive::{TriggerItemBase, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase};

// ============== // Trigger // =============== //

/// <https://wiki.factorio.com/Types/Trigger>
#[derive(Debug, Clone)]
pub enum Trigger {
    Direct(DirectTriggerItem),
    Area(AreaTriggerItem),
    Line(LineTriggerItem),
    Cluster(ClusterTriggerItem)
}

/// <https://wiki.factorio.com/Types/TriggerItem>
#[derive(Debug, Clone)]
pub struct TriggerItem {
    entity_flags: EntityPrototypeFlags, // Default: all flags
    ignore_collision_condition: bool, // Default: false
    trigger_target_mask: TriggerTargetMask, // Default: all flags
    repeat_count: u32, // Default: 1
    probability: f32, // Default: 1
    collision_mask: CollisionMask, // Default: all
    action_delivery: Option<Vec<TriggerDelivery>>,
    force: ForceCondition // Default: all forces
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
#[derive(Debug, Clone, TriggerItemBase)]
pub struct DirectTriggerItem {
    base: TriggerItem,
    filter_enabled: bool // Default: false
}

/// <https://wiki.factorio.com/Types/AreaTriggerItem>
#[derive(Debug, Clone, TriggerItemBase)]
pub struct AreaTriggerItem {
    base: TriggerItem,
    radius: f64,
    trigger_from_target: bool, // Default: false
    target_entities: bool, // Default: true
    show_in_tooltip: bool, // Default: true
    collision_mode: CollisionMode // Default: "distance-from-collision-box"
}

/// <https://wiki.factorio.com/Types/LineTriggerItem>
#[derive(Debug, Clone, TriggerItemBase)]
pub struct LineTriggerItem {
    base: TriggerItem,
    range: f64,
    width: f64,
    range_effects: Option<TriggerEffect>
}

/// <https://wiki.factorio.com/Types/ClusterTriggerItem>
#[derive(Debug, Clone, TriggerItemBase)]
pub struct ClusterTriggerItem {
    base: TriggerItem,
    cluster_count: f64, // Must be at least 2
    distance: f32,
    distance_deviation: f32 // Default: 0
}

// ========== // TriggerDelivery // =========== //

/// <https://wiki.factorio.com/Types/TriggerDelivery>
#[derive(Debug, Clone)]
pub enum TriggerDelivery {
    Instant(InstantTriggerDelivery),
    Projectile(ProjectileTriggerDelivery),
    FlameThrowerExplosion(FlameThrowerExplosionTriggerDelivery),
    Beam(BeamTriggerDelivery),
    Stream(StreamTriggerDelivery),
    Artillery(ArtilleryTriggerDelivery)
}

/// <https://wiki.factorio.com/Types/InstantTriggerDelivery>
#[derive(Debug, Clone)]
pub struct InstantTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>
}

/// <https://wiki.factorio.com/Types/ProjectileTriggerDelivery>
#[derive(Debug, Clone)]
pub struct ProjectileTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    projectile: String,
    starting_speed: f32,
    starting_speed_deviation: f32, // Default: 0
    direction_deviation: f32, // Default: 0
    range_deviation: f32, // Default: 0
    max_range: f64, // Default: 1000
    min_range: f64 // Default: 0
}

/// <https://wiki.factorio.com/Types/FlameThrowerExplosionTriggerDelivery>
#[derive(Debug, Clone)]
pub struct FlameThrowerExplosionTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    explosion: String,
    starting_distance: f64,
    direction_deviation: f32, // Default: 0
    speed_deviation: f64, // Default: 0
    starting_frame_fraction_deviation: f64, // Default: 0
    projectile_starting_speed: f64 // Default: 1
}

/// <https://wiki.factorio.com/Types/BeamTriggerDelivery>
#[derive(Debug, Clone)]
pub struct BeamTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    beam: String, // Name of Beam prototype
    add_to_shooter: bool, // Default: true
    max_length: u32, // Default: 0
    duration: u32, // Default: 0
    source_offset: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/StreamTriggerDelivery>
#[derive(Debug, Clone)]
pub struct StreamTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    stream: String, // Name of FluidStream prototype
    source_offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/ArtilleryTriggerDelivery>
#[derive(Debug, Clone)]
pub struct ArtilleryTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    projectile: String, // Name of ArtilleryProjectile prototype
    starting_speed: f32,
    starting_speed_deviation: f32, // Default: 0
    direction_deviation: f32, // Default: 0
    range_deviation: f32, // Default: 0
    trigger_fired_artillery: bool // Default: false
}

// ============ // TriggerEffect // =========== //

/// <https://wiki.factorio.com/Types/TriggerEffect>
#[derive(Debug, Clone)]
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
    DestoryCliffs(DestroyCliffsTriggerEffectItem),
    ShowExplosionOnChart(ShowExplosionOnChartTriggerEffectItem),
    InsertItem(InsertItemTriggerEffectItem),
    Script(ScriptTriggerEffectItem),
    SetTile(SetTileTriggerEffectItem),
    InvokeTileTrigger(InvokeTileEffectTriggerEffectItem),
    DestoryDecoratives(DestroyDecorativesTriggerEffectItem),
    CameraEffect(CameraEffectTriggerEffectItem),
}

/// <https://wiki.factorio.com/Types/TriggerEffectItem>
#[derive(Debug, Clone)]
pub struct TriggerEffectItem {
    repeat_count: u16, // Default: 1
    repeat_count_deviation: u16, // Default: 0
    probability: f32, // Default: 1 // 0 < value <= 1
    affects_target: bool, // Default: false
    show_in_tooltip: bool, // Default: true // Default: false in some cases
    damage_type_filters: Option<DamageTypeFilters>
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
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct DamageTriggerEffectItem {
    base: TriggerEffectItem,
    damage: DamagePrototype,
    apply_damage_to_trees: bool, // Default: true
    vaporize: bool, // Default: false
    lower_distance_threshold: u16, // Default: u16::MAX
    upper_distance_threshold: u16, // Default: u16::MAX
    lower_damage_modifier: f32, // Default: 1
    upper_damage_modifier: f32  // Default: 1
}

/// <https://wiki.factorio.com/Types/CreateEntityTriggerEffectItem>
#[derive(Debug, Clone)]
pub struct CreateEntityTriggerEffect {
    entity_name: String, // Entity name
    offset_deviation: Option<BoundingBox>,
    trigger_created_entity: bool, // Default: false
    check_buildability: bool, // Default: false
    // Override default in constructor
    show_in_tooltip: bool, // Default: false
    tile_collision_mask: Option<CollisionMask>,
    offsets: Option<Vec<Factorio2DVector>>
}

/// <https://wiki.factorio.com/Types/CreateEntityTriggerEffectItem>
pub trait CreateEntityTriggerEffectItemBase {
    fn entity_name(&self) -> &String;
    fn offset_deviation(&self) -> &Option<BoundingBox>;
    fn trigger_created_entity(&self) -> bool;
    fn check_buildability(&self) -> bool;
    fn show_in_tooltip(&self) -> bool;
    fn tile_collision_mask(&self) -> &Option<CollisionMask>;
    fn offsets(&self) -> &Option<Vec<Factorio2DVector>>;
}

/// <https://wiki.factorio.com/Types/CreateEntityTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateEntityTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect
}

/// <https://wiki.factorio.com/Types/CreateExplosionTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateExplosionTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect,
    max_movement_distance: f32, // Default: -1
    max_movement_distance_deviation: f32, // Default: 0
    inherit_movement_distance_from_projectile: bool, // Default: false
    cycle_while_moving: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CreateFireTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateFireTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect,
    initial_ground_flame_count: u8 // Default: u8::MAX
}

/// <https://wiki.factorio.com/Types/CreateSmokeTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateSmokeTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect,
    initial_height: f32, // Default: 0
    speed: Option<Factorio2DVector>,
    speed_multiplier: f32, // Default: 0
    speed_multiplier_deviation: f32, // Default: 0
    starting_frame: f32, // Default: 0 // Why is it f32?
    starting_frame_deviation: f32, // Default: 0
    starting_frame_speed: f32, // Default: 0
    starting_frame_speed_deviation: f32, // Default: 0
    speed_from_center: f32, // Default: 0
    speed_from_center_deviation: f32 // Default: 0
}

/// <https://wiki.factorio.com/Types/CreateTrivialSmokeEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct CreateTrivialSmokeEffectItem {
    base: TriggerEffectItem,
    smoke_name: String, // Name of TrivialSmoke prototype
    offset_deviation: Option<BoundingBox>,
    offsets: Option<Vec<Factorio2DVector>>,
    initial_height: f32, // Default: 0
    max_radius: f32, // Default: 0
    speed: Factorio2DVector, // Default: (0, 0)
    speed_multiplier: f32, // Default: 0
    speed_multiplier_deviation: f32, // Default: 0
    starting_frame: f32, // Default: 0
    starting_frame_deviation: f32, // Default: 0
    starting_frame_speed: f32, // Default: 0
    starting_frame_speed_deviation: f32, // Default: 0
    speed_from_center: f32, // Default: 0
    speed_from_center_deviation: f32 // Default: 0
}

/// <https://wiki.factorio.com/Types/CreateParticleTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct CreateParticleTriggerEffectItem {
    base: TriggerEffectItem,
    particle_name: String, // Name of Particle prototype
    initial_height: f32,
    offset_deviation: Option<BoundingBox>,
    // show_in_tooltip: Default: false // Override in constructor
    tile_collision_mask: Option<CollisionMask>,
    offsets: Option<Vec<Factorio2DVector>>,
    initial_height_deviation: f32, // Default: 0
    initial_vertical_speed: f32, // Default: 0
    initial_vertical_speed_deviation: f32, // Default: 0
    speed_from_center: f32, // Default: 0
    speed_from_center_deviation: f32, // Default: 0
    frame_speed: f32, // Default: 1
    frame_speed_deviation: f32, // Default: 0
    tail_length: u8, // Default: 0 // Silently capped to maximum fo 100
    tail_length_deviation: u8, // Default: 0
    tail_width: f32, // Default: 1
    rotate_offsets: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CreateStickerTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct CreateStickerTriggerEffectItem {
    base: TriggerEffectItem,
    stricker: String, // Name of Sticker prototype
    // show_in_tooltip: Default: false // Override in constructor
    trigger_created_entity: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CreateDecorativesTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct CreateDecorativesTriggerEffectItem {
    base: TriggerEffectItem,
    decorative: String, // name of Decorative prototype
    spawn_max: u16,
    spawn_min_radius: f32,
    spawn_max_radius: f32, // Limited to < 24
    spawn_min: u16, // Default: u16
    radius_curve: f32, // Default: 0.5
    apply_projection: bool, // Default: false
    spread_evenly: bool // Default: false
}

/// <https://wiki.factorio.com/Types/NestedTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct NestedTriggerEffectItem {
    base: TriggerEffectItem,
    action: Trigger
}

/// <https://wiki.factorio.com/Types/PlaySoundTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct PlaySoundTriggerEffectItem {
    base: TriggerEffectItem,
    sound: Sound,
    // Negative values are silently clamped to 0
    min_distance: f32, // Default: 0
    max_distance: f32, // Default: 1e21
    volume_modifier: f32, // Default: 1
    audible_distance_modifier: f32, // Default: 1
    play_on_target_position: bool // Default: false
}

/// <https://wiki.factorio.com/Types/PushBackTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct PushBackTriggerEffectItem {
    base: TriggerEffectItem,
    distance: f32
}

/// <https://wiki.factorio.com/Types/DestroyCliffsTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct DestroyCliffsTriggerEffectItem {
    base: TriggerEffectItem,
    radius: f32,
    explosion: Option<String>, // Name of an entity
}

/// <https://wiki.factorio.com/Types/ShowExplosionOnChartTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    base: TriggerEffectItem,
    scale: f32
}

/// <https://wiki.factorio.com/Types/InsertItemTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct InsertItemTriggerEffectItem {
    base: TriggerEffectItem,
    item: String, // Name of an item
    count: u32 // Default: 1
}

/// <https://wiki.factorio.com/Types/ScriptTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct ScriptTriggerEffectItem {
    base: TriggerEffectItem,
    effect_id: String
}

/// <https://wiki.factorio.com/Types/SetTileTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct SetTileTriggerEffectItem {
    base: TriggerEffectItem,
    tile_name: String, // Name of a prototype
    radius: f32,
    apply_projection: bool, // Default: false
    tile_collision_mask: CollisionMask // Default: none
}

/// <https://wiki.factorio.com/Types/InvokeTileEffectTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct InvokeTileEffectTriggerEffectItem {
    base: TriggerEffectItem,
    tile_collision_mask: Option<CollisionMask>
}

/// <https://wiki.factorio.com/Types/DestroyDecorativesTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct DestroyDecorativesTriggerEffectItem {
    base: TriggerEffectItem,
    radius: f32,
    from_render_layer: RenderLayer, // Default: first layer
    to_render_layer: RenderLayer, // Default: last layer
    include_soft_decoratives: bool, // Default: false
    include_decals: bool, // Default: false
    invoke_decorative_trigger: bool, // Default: true
    decoratives_with_trigger_only: bool // Default: false
}

/// <https://wiki.factorio.com/Types/CameraEffectTriggerEffectItem>
#[derive(Debug, Clone, TriggerEffectItemBase)]
pub struct CameraEffectTriggerEffectItem {
    base: TriggerEffectItem,
    effect: String,
    duration: u8,
    ease_in_duration: u8, // Default: 0
    ease_out_duration: u8, // Default: 0
    delay: u8, // Default: 0
    full_strength_max_distance: u16, // Default: 0
    max_distance: u16, // Default: 0
    strength: f32, // Default: 0
}

// =============== // Other // ================ //

/// <https://wiki.factorio.com/Types/FootstepTriggerEffectList>
pub type FootstepTriggerEffectList = Vec<FootstepTriggerEffect>;

/// <https://wiki.factorio.com/Types/FootstepTriggerEffectList>
#[derive(Debug, Clone)]
pub struct FootstepTriggerEffect {
    actions: Vec<CreateParticleTriggerEffectItem>,
    use_as_default: bool, // Default: false
    tiles: Vec<String>, // (Names) Name of tile
}

/// <https://wiki.factorio.com/Types/TriggerTargetMask>
#[derive(Debug, Clone)]
pub enum TriggerTargetMask {
    Everything,
    Specific(Vec<String>)
}
