use std::collections::HashMap;
use crate::prototypes::{PrototypeBaseSpec, Prototype, PrototypeBase};
use crate::types::{Color, Sound, DaytimeColorLookupTable, Factorio2DVector, TriggerTargetMask, FileName, SimulationDefinition};
use crate::concepts::LocalisedString;

/// <https://wiki.factorio.com/Prototype/UtilityConstants>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct UtilityConstants {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_button_background_color: Color,
    building_buildable_too_far_tint: Color,
    building_buildable_tint: Color,
    building_not_buildable_tint: Color,
    building_ignorable_tint: Color,
    building_no_tint: Color,
    ghost_tint: Color,
    tile_ghost_tint: Color,
    equipment_default_background_color: Color,
    equipment_default_background_border_color: Color,
    equipment_default_grabbed_background_color: Color,
    turret_range_visualization_color: Color,
    capsule_range_visualization_color: Color,
    artillery_range_visualization_color: Color,
    train_no_path_color: Color,
    train_destination_full_color: Color,
    chart: UtiliyConstantsChart,
    default_player_force_color: Color,
    item_outline_radiusdefault_enemy_force_color: Color,
    default_other_force_color: Color,
    deconstruct_mark_tint: Color,
    rail_planner_count_button_color: Color,
    count_button_size: i32,
    zoom_to_world_can_use_nightvision: bool,
    zoom_to_world_effect_strength: f32,
    max_terrain_building_size: u8,
    small_area_size: f32,
    medium_area_size: f32,
    small_blueprint_area_size: f32,
    medium_blueprint_area_size: f32,
    enabled_recipe_slot_tint: Color,
    disabled_recipe_slot_tint: Color,
    disabled_recipe_slot_background_tint: Color,
    forced_enabled_recipe_slot_background_tint: Color,
    rail_segment_colors: Vec<Color>,
    player_colors: Vec<UtilityConstantsPlayerColor>, // item with `name` == default must exist and be the first item in array
    server_command_console_chat_color: Color,
    script_command_console_chat_color: Color,
    default_alert_icon_scale: f32,
    default_alert_icon_shift_by_type: Option<HashMap<String, Factorio2DVector>>,
    default_alert_icon_scale_by_type: Option<HashMap<String, f32>>,
    daytime_color_lookup: DaytimeColorLookupTable,
    zoom_to_world_daytime_color_lookup: DaytimeColorLookupTable,
    checkerboard_white: Color,
    checkerboard_black: Color,
    item_outline_color: Color,
    item_outline_radius: f32,
    item_outline_inset: f32,
    item_outline_sharpness: f32,
    filter_outline_color: Color,
    icon_shadow_radius: f32,
    icon_shadow_inset: f32,
    icon_shadow_sharpness: f32,
    icon_shadow_color: Color,
    clipboard_history_size: u32,
    recipe_step_limit: u32,
    manual_rail_building_reach_modifier: f64,
    train_temporary_stop_wait_time: u32,
    train_time_wait_condition_default: u32,
    train_inactivity_wait_condition_default: u32,
    default_trigger_target_mask_by_type: Option<HashMap<String, TriggerTargetMask>>,
    unit_group_pathfind_resolution: i8,
    unit_group_max_pursue_distance: f64,
    dynamic_recipe_overload_factor: f64,
    minimum_recipe_overload_multiplier: u32,
    maximum_recipe_overload_multiplier: u32,
    tree_leaf_distortion_strength_far: Factorio2DVector,
    tree_leaf_distortion_distortion_far: Factorio2DVector,
    tree_leaf_distortion_speed_far: Factorio2DVector,
    tree_leaf_distortion_strength_near: Factorio2DVector,
    tree_leaf_distortion_distortion_near: Factorio2DVector,
    tree_leaf_distortion_speed_near: Factorio2DVector,
    tree_shadow_roughness: f32,
    tree_shadow_speed: f32,
    missing_preview_sprite_location: FileName,
    main_menu_background_image_location: FileName,
    main_menu_simulations: HashMap<String, SimulationDefinition>,
    main_menu_background_vignette_intensity: f32,
    main_menu_background_vignette_sharpness: f32,
    default_scorch_mark_color: Color,
    train_button_hovered_tint: Color,
    select_group_row_count: u32, // Range: [1, 100]
    select_slot_row_count: u32, // Range: [1, 100]
    inventory_width: u32, // Range: [1, 100]
    module_inventory_width: u32, // Range: [1, 100]
    tooltip_monitor_edge_border: i32, // Must be >= 1
    normalised_achievement_icon_size: u32, // Must be >= 1
    tutorial_notice_icon_size: u32, // Must be >= 1
    flying_text_ttl: u32, // Must be >= 1
    bonus_gui_ordering: HashMap<String, String>,
    train_path_finding: UtilityConstantsTrainPathFinding,
    map_editor: UtilityConstantsMapEditor,
    color_filters: Vec<UtilityConstantColorFilter>,
    entity_renderer_search_box_limits: UtilityConstantsEntityRendererSerahcBoxLimits,
    light_renderer_search_distance_limit: u8
}

#[derive(Debug)]
pub struct UtiliyConstantsChart {
    electric_lines_color: Color,
    electric_lines_color_switch_enabled: Color,
    electric_lines_color_switch_disabled: Color,
    electric_power_pole_color: Color,
    switch_color: Color,
    electric_line_width: f64,
    electric_line_minimum_absolute_width: f64,
    turret_range_color: Color,
    artillery_range_color: Color,
    default_friendly_color: Color,
    default_enemy_color: Color,
    rail_color: Color,
    entity_ghost_color: Color,
    vehicle_outer_color: Color,
    vehicle_outer_color_selected: Color,
    vehicle_inner_color: Color,
    vehicle_wagon_connection_color: Color,
    resource_outline_selection_color: Color,
    chart_train_stop_text_color: Color,
    chart_train_stop_disabled_text_color: Color,
    chart_train_stop_full_text_color: Color,
    red_signal_color: Color,
    green_signal_color: Color,
    blue_signal_color: Color,
    yellow_signal_color: Color,
    chart_deconstruct_tint: Color,
    default_friendly_color_by_type: Option<HashMap<String, Color>>,
    default_color_by_type: Option<HashMap<String, Color>>,
    explosion_visualization_duration: u32,
    train_path_color: Color,
    train_preview_path_outline_color: Color,
    train_current_path_outline_color: Color,
    custom_tag_scale: f32, // Default: 0.6
    custom_tag_selected_overlay_tint: Color
}

#[derive(Debug)]
pub struct UtilityConstantsPlayerColor {
    name: String,
    player_color: Color,
    chat_color: Color
}

#[derive(Debug)]
pub struct UtilityConstantsTrainPathFinding {
    train_stop_penalty: u32,
    stopped_manually_controlled_train_penalty: u32,
    stopped_manually_controlled_train_without_passenger_penalty: u32,
    signal_reserved_by_circuit_network_penalty: u32,
    train_in_station_penalty: u32,
    train_in_station_with_no_other_valid_stops_in_schedule: u32,
    train_arriving_to_station_penalty: u32,
    train_arriving_to_signal_penalty: u32,
    train_waiting_at_signal_penalty: u32,
    train_waiting_at_signal_tick_multiplier_penalty: f32, // Must be >= 0
    train_with_no_path_penalty: u32,
    train_auto_without_schedule_penalty: u32
}

#[derive(Debug)]
pub struct UtilityConstantsMapEditor {
    clone_editor_copy_source_color: Color,
    clone_editor_copy_destination_allowed_color: Color,
    clone_editor_copy_destination_not_allowed_color: Color,
    clone_editor_brush_source_color: Color,
    clone_editor_brush_destination_color: Color,
    clone_editor_brush_cursor_preview_tint: Color,
    clone_editor_brush_world_preview_tint: Color,
    script_editor_select_area_color: Color,
    script_editor_drag_area_color: Color,
    force_editor_select_area_color: Color,
    cliff_editor_remove_cliffs_color: Color,
    tile_editor_selection_preview_tint: Color,
    tile_editor_area_selection_color: Color,
    decorative_editor_selection_preview_tint: Color,
    tile_editor_selection_preview_radius: u8,
    decorative_editor_selection_preview_radius: u8
}

#[derive(Debug)]
pub struct UtilityConstantColorFilter {
    name: String,
    localised_name: LocalisedString,
    matrix: [[f32; 4]; 4]
}

#[derive(Debug)]
pub struct UtilityConstantsEntityRendererSerahcBoxLimits {
    left: u8, // Range [6, 15]
    top: u8, // Range [3, 15]
    right: u8, // Range [3, 15]
    bottom: u8, // Range [4, 15]
}

/// <https://wiki.factorio.com/Prototype/UtilitySounds>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct UtilitySounds {
    name: String,
    prototype_base: PrototypeBaseSpec,
    gui_click: Sound,
    list_box_click: Sound,
    build_small: Sound,
    build_medium: Sound,
    build_large: Sound,
    cannot_build: Sound,
    build_blueprint_small: Sound,
    build_blueprint_medium: Sound,
    build_blueprint_large: Sound,
    deconstruct_small: Sound,
    deconstruct_medium: Sound,
    deconstruct_big: Sound,
    deconstruct_robot: Sound,
    rotated_small: Sound,
    rotated_medium: Sound,
    rotated_big: Sound,
    axe_mining_ore: Sound,
    mining_wood: Sound,
    axe_fighting: Sound,
    alert_destroyed: Sound,
    console_message: Sound,
    scenario_message: Sound,
    new_objective: Sound,
    game_lost: Sound,
    game_won: Sound,
    metal_walking_sound: Sound,
    research_completed: Sound,
    default_manual_repair: Sound,
    crafting_finished: Sound,
    inventory_click: Sound,
    inventory_move: Sound,
    clear_cursor: Sound,
    armor_insert: Sound,
    armor_remove: Sound,
    achievement_unlocked: Sound,
    wire_connect_pole: Sound,
    wire_disconnect: Sound,
    wire_pickup: Sound,
    tutorial_notice: Sound,
    smart_pipette: Sound,
    switch_gun: Sound,
    picked_up_item: Sound,
    blueprint_selection_ended: Sound,
    blueprint_selection_started: Sound,
    deconstruction_selection_started: Sound,
    deconstruction_selection_ended: Sound,
    cancel_deconstruction_selection_started: Sound,
    cancel_deconstruction_selection_ended: Sound,
    upgrade_selection_started: Sound,
    upgrade_selection_ended: Sound,
    copy_activated: Sound,
    cut_activated: Sound,
    paste_activated: Sound,
    item_deleted: Sound,
    entity_settings_pasted: Sound,
    entity_settings_copied: Sound,
    item_spawned: Sound,
    confirm: Sound,
    undo: Sound,
    drop_item: Sound,
    rail_plan_start: Sound
}
