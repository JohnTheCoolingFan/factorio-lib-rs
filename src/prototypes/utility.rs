use super::additional_types::{
    Animation, BoxSpecification, Color, DaytimeColorLookupTable, Factorio2DVector, FileName,
    SimulationDefinition, Sound, Sprite, TriggerTargetMask,
};
use super::{DataTable, LocalisedString};
use crate::prototypes::{
    GetPrototype, Prototype, PrototypeBase, PrototypeBaseSpec, PrototypeFromLua, PrototypeType,
};
use factorio_lib_rs_derive::DataTableAccessable;
use mlua::prelude::*;
use std::collections::HashMap;

/// <https://wiki.factorio.com/Prototype/UtilityConstants>
#[derive(Debug, Clone, Prototype, PrototypeBase, DataTableAccessable, PrototypeFromLua)]
#[data_table(utility_constants)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct UtilityConstants {
    pub name: String,
    #[use_self_forced]
    pub prototype_base: PrototypeBaseSpec,
    pub entity_button_background_color: Color,
    pub building_buildable_too_far_tint: Color,
    pub building_buildable_tint: Color,
    pub building_not_buildable_tint: Color,
    pub building_ignorable_tint: Color,
    pub building_no_tint: Color,
    pub ghost_tint: Color,
    pub tile_ghost_tint: Color,
    pub equipment_default_background_color: Color,
    pub equipment_default_background_border_color: Color,
    pub equipment_default_grabbed_background_color: Color,
    pub turret_range_visualization_color: Color,
    pub capsule_range_visualization_color: Color,
    pub artillery_range_visualization_color: Color,
    pub train_no_path_color: Color,
    pub train_destination_full_color: Color,
    pub chart: UtilityConstantsChart,
    pub default_player_force_color: Color,
    pub item_outline_radiusdefault_enemy_force_color: Color,
    pub default_other_force_color: Color,
    pub deconstruct_mark_tint: Color,
    pub rail_planner_count_button_color: Color,
    pub count_button_size: i32,
    pub zoom_to_world_can_use_nightvision: bool,
    pub zoom_to_world_effect_strength: f32,
    pub max_terrain_building_size: u8,
    pub small_area_size: f32,
    pub medium_area_size: f32,
    pub small_blueprint_area_size: f32,
    pub medium_blueprint_area_size: f32,
    pub enabled_recipe_slot_tint: Color,
    pub disabled_recipe_slot_tint: Color,
    pub disabled_recipe_slot_background_tint: Color,
    pub forced_enabled_recipe_slot_background_tint: Color,
    pub rail_segment_colors: Vec<Color>,
    pub player_colors: Vec<UtilityConstantsPlayerColor>, // item with `name` == default must exist and be the first item in array
    pub server_command_console_chat_color: Color,
    pub script_command_console_chat_color: Color,
    pub default_alert_icon_scale: f32,
    pub default_alert_icon_shift_by_type: Option<HashMap<String, Factorio2DVector>>,
    pub default_alert_icon_scale_by_type: Option<HashMap<String, f32>>,
    pub daytime_color_lookup: DaytimeColorLookupTable,
    pub zoom_to_world_daytime_color_lookup: DaytimeColorLookupTable,
    pub checkerboard_white: Color,
    pub checkerboard_black: Color,
    pub item_outline_color: Color,
    pub item_outline_radius: f32,
    pub item_outline_inset: f32,
    pub item_outline_sharpness: f32,
    pub filter_outline_color: Color,
    pub icon_shadow_radius: f32,
    pub icon_shadow_inset: f32,
    pub icon_shadow_sharpness: f32,
    pub icon_shadow_color: Color,
    pub clipboard_history_size: u32,
    pub recipe_step_limit: u32,
    pub manual_rail_building_reach_modifier: f64,
    pub train_temporary_stop_wait_time: u32,
    pub train_time_wait_condition_default: u32,
    pub train_inactivity_wait_condition_default: u32,
    pub default_trigger_target_mask_by_type: Option<HashMap<String, TriggerTargetMask>>,
    pub unit_group_pathfind_resolution: i8,
    pub unit_group_max_pursue_distance: f64,
    pub dynamic_recipe_overload_factor: f64,
    pub minimum_recipe_overload_multiplier: u32,
    pub maximum_recipe_overload_multiplier: u32,
    pub tree_leaf_distortion_strength_far: Factorio2DVector,
    pub tree_leaf_distortion_distortion_far: Factorio2DVector,
    pub tree_leaf_distortion_speed_far: Factorio2DVector,
    pub tree_leaf_distortion_strength_near: Factorio2DVector,
    pub tree_leaf_distortion_distortion_near: Factorio2DVector,
    pub tree_leaf_distortion_speed_near: Factorio2DVector,
    pub tree_shadow_roughness: f32,
    pub tree_shadow_speed: f32,
    pub missing_preview_sprite_location: FileName,
    pub main_menu_background_image_location: FileName,
    pub main_menu_simulations: HashMap<String, SimulationDefinition>,
    pub main_menu_background_vignette_intensity: f32,
    pub main_menu_background_vignette_sharpness: f32,
    pub default_scorch_mark_color: Color,
    pub train_button_hovered_tint: Color,
    pub select_group_row_count: u32,           // Range: [1, 100]
    pub select_slot_row_count: u32,            // Range: [1, 100]
    pub inventory_width: u32,                  // Range: [1, 100]
    pub module_inventory_width: u32,           // Range: [1, 100]
    pub tooltip_monitor_edge_border: i32,      // Must be >= 1
    pub normalised_achievement_icon_size: u32, // Must be >= 1
    pub tutorial_notice_icon_size: u32,        // Must be >= 1
    pub flying_text_ttl: u32,                  // Must be >= 1
    pub bonus_gui_ordering: HashMap<String, String>,
    pub train_path_finding: UtilityConstantsTrainPathFinding,
    pub map_editor: UtilityConstantsMapEditor,
    pub color_filters: Vec<UtilityConstantColorFilter>,
    pub entity_renderer_search_box_limits: UtilityConstantsEntityRendererSearchBoxLimits,
    pub light_renderer_search_distance_limit: u8,
}

impl UtilityConstants {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if let Some(v) = self.player_colors.get(0) {
            if v.name != "default" {
                return Err(LuaError::FromLuaConversionError {
                    from: "table",
                    to: "UtilityConstants",
                    message: Some("`player_colors[0].name` must be \"default\"".into()),
                });
            }
        } else {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`player_colors[0]` must exist".into()),
            });
        }
        if self.tooltip_monitor_edge_border < 1 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`tooltip_monitor_edge_border` must be >= 1".into()),
            });
        }
        if self.normalised_achievement_icon_size < 1 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`normalised_achievement_icon_size` must be >= 1".into()),
            });
        }
        if self.tutorial_notice_icon_size < 1 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`tutorial_notice_icon_size` must be >= 1".into()),
            });
        }
        if self.flying_text_ttl < 1 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`flying_text_ttl` must be >= 1".into()),
            });
        }
        if !(1..=100).contains(&self.select_group_row_count) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`select_group_row_count` must be in a range [1; 100]".into()),
            });
        }
        if !(1..=100).contains(&self.select_slot_row_count) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`select_slot_row_count` must be in a range [1; 100]".into()),
            });
        }
        if !(1..=100).contains(&self.inventory_width) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`inventory_width` must be in a range [1; 100]".into()),
            });
        }
        if !(1..=100).contains(&self.module_inventory_width) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstants",
                message: Some("`module_inventory_width` must be in a range [1; 100]".into()),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UtilityConstantsChart {
    pub electric_lines_color: Color,
    pub electric_lines_color_switch_enabled: Color,
    pub electric_lines_color_switch_disabled: Color,
    pub electric_power_pole_color: Color,
    pub switch_color: Color,
    pub electric_line_width: f64,
    pub electric_line_minimum_absolute_width: f64,
    pub turret_range_color: Color,
    pub artillery_range_color: Color,
    pub pollution_color: Color,
    pub default_friendly_color: Color,
    pub default_enemy_color: Color,
    pub rail_color: Color,
    pub entity_ghost_color: Color,
    pub vehicle_outer_color: Color,
    pub vehicle_outer_color_selected: Color,
    pub vehicle_inner_color: Color,
    pub vehicle_wagon_connection_color: Color,
    pub resource_outline_selection_color: Color,
    pub chart_train_stop_text_color: Color,
    pub chart_train_stop_disabled_text_color: Color,
    pub chart_train_stop_full_text_color: Color,
    pub red_signal_color: Color,
    pub green_signal_color: Color,
    pub blue_signal_color: Color,
    pub yellow_signal_color: Color,
    pub chart_deconstruct_tint: Color,
    pub default_friendly_color_by_type: Option<HashMap<String, Color>>,
    pub default_color_by_type: Option<HashMap<String, Color>>,
    pub explosion_visualization_duration: u32,
    pub train_path_color: Color,
    pub train_preview_path_outline_color: Color,
    pub train_current_path_outline_color: Color,
    #[default(0.6_f32)]
    pub custom_tag_scale: f32, // Default: 0.6
    pub custom_tag_selected_overlay_tint: Color,
}

#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UtilityConstantsPlayerColor {
    pub name: String,
    pub player_color: Color,
    pub chat_color: Color,
}

#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct UtilityConstantsTrainPathFinding {
    pub train_stop_penalty: u32,
    pub stopped_manually_controlled_train_penalty: u32,
    pub stopped_manually_controlled_train_without_passenger_penalty: u32,
    pub signal_reserved_by_circuit_network_penalty: u32,
    pub train_in_station_penalty: u32,
    pub train_in_station_with_no_other_valid_stops_in_schedule: u32,
    pub train_arriving_to_station_penalty: u32,
    pub train_arriving_to_signal_penalty: u32,
    pub train_waiting_at_signal_penalty: u32,
    pub train_waiting_at_signal_tick_multiplier_penalty: f32, // Must be >= 0
    pub train_with_no_path_penalty: u32,
    pub train_auto_without_schedule_penalty: u32,
}

impl UtilityConstantsTrainPathFinding {
    fn post_extr_fn(&self, _lua: &mlua::Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.train_waiting_at_signal_tick_multiplier_penalty < 0.0 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstantsTrainPathFinding",
                message: Some(
                    "`train_waiting_at_signal_tick_multiplier_penalty` must be >= 0".into(),
                ),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UtilityConstantsMapEditor {
    pub clone_editor_copy_source_color: Color,
    pub clone_editor_copy_destination_allowed_color: Color,
    pub clone_editor_copy_destination_not_allowed_color: Color,
    pub clone_editor_brush_source_color: Color,
    pub clone_editor_brush_destination_color: Color,
    pub clone_editor_brush_cursor_preview_tint: Color,
    pub clone_editor_brush_world_preview_tint: Color,
    pub script_editor_select_area_color: Color,
    pub script_editor_drag_area_color: Color,
    pub force_editor_select_area_color: Color,
    pub cliff_editor_remove_cliffs_color: Color,
    pub tile_editor_selection_preview_tint: Color,
    pub tile_editor_area_selection_color: Color,
    pub decorative_editor_selection_preview_tint: Color,
    pub tile_editor_selection_preview_radius: u8,
    pub decorative_editor_selection_preview_radius: u8,
}

#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UtilityConstantColorFilter {
    pub name: String,
    pub localised_name: LocalisedString,
    pub matrix: [[f32; 4]; 4],
}

#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct UtilityConstantsEntityRendererSearchBoxLimits {
    pub left: u8,   // Range [6, 15]
    pub top: u8,    // Range [3, 15]
    pub right: u8,  // Range [3, 15]
    pub bottom: u8, // Range [4, 15]
}

impl UtilityConstantsEntityRendererSearchBoxLimits {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if !(6..=15).contains(&self.left) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstantsEntityRendererSearchBoxLimits",
                message: Some("`left` msut be in range [6; 15]".into()),
            });
        }
        if !(6..=15).contains(&self.top) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstantsEntityRendererSearchBoxLimits",
                message: Some("`top` msut be in range [3; 15]".into()),
            });
        }
        if !(6..=15).contains(&self.right) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstantsEntityRendererSearchBoxLimits",
                message: Some("`right` msut be in range [3; 15]".into()),
            });
        }
        if !(6..=15).contains(&self.bottom) {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "UtilityConstantsEntityRendererSearchBoxLimits",
                message: Some("`bottom` msut be in range [4; 15]".into()),
            });
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Prototype/UtilitySounds>
#[derive(Debug, Clone, Prototype, PrototypeBase, DataTableAccessable, PrototypeFromLua)]
#[data_table(utility_sounds)]
pub struct UtilitySounds {
    pub name: String,
    #[use_self_forced]
    pub prototype_base: PrototypeBaseSpec,
    pub gui_click: Sound,
    pub list_box_click: Sound,
    pub build_small: Sound,
    pub build_medium: Sound,
    pub build_large: Sound,
    pub cannot_build: Sound,
    pub build_blueprint_small: Sound,
    pub build_blueprint_medium: Sound,
    pub build_blueprint_large: Sound,
    pub deconstruct_small: Sound,
    pub deconstruct_medium: Sound,
    pub deconstruct_big: Sound,
    pub deconstruct_robot: Sound,
    pub rotated_small: Sound,
    pub rotated_medium: Sound,
    pub rotated_big: Sound,
    pub axe_mining_ore: Sound,
    pub mining_wood: Sound,
    pub axe_fighting: Sound,
    pub alert_destroyed: Sound,
    pub console_message: Sound,
    pub scenario_message: Sound,
    pub new_objective: Sound,
    pub game_lost: Sound,
    pub game_won: Sound,
    pub metal_walking_sound: Sound,
    pub research_completed: Sound,
    pub default_manual_repair: Sound,
    pub crafting_finished: Sound,
    pub inventory_click: Sound,
    pub inventory_move: Sound,
    pub clear_cursor: Sound,
    pub armor_insert: Sound,
    pub armor_remove: Sound,
    pub achievement_unlocked: Sound,
    pub wire_connect_pole: Sound,
    pub wire_disconnect: Sound,
    pub wire_pickup: Sound,
    pub tutorial_notice: Sound,
    pub smart_pipette: Sound,
    pub switch_gun: Sound,
    pub picked_up_item: Sound,
    pub blueprint_selection_ended: Sound,
    pub blueprint_selection_started: Sound,
    pub deconstruction_selection_started: Sound,
    pub deconstruction_selection_ended: Sound,
    pub cancel_deconstruction_selection_started: Sound,
    pub cancel_deconstruction_selection_ended: Sound,
    pub upgrade_selection_started: Sound,
    pub upgrade_selection_ended: Sound,
    pub copy_activated: Sound,
    pub cut_activated: Sound,
    pub paste_activated: Sound,
    pub item_deleted: Sound,
    pub entity_settings_pasted: Sound,
    pub entity_settings_copied: Sound,
    pub item_spawned: Sound,
    pub confirm: Sound,
    pub undo: Sound,
    pub drop_item: Sound,
    pub rail_plan_start: Sound,
}

/// <https://wiki.factorio.com/Prototype/UtilitySprites>
#[derive(Debug, Clone, Prototype, PrototypeBase, DataTableAccessable, PrototypeFromLua)]
#[data_table(utility_sprites)]
pub struct UtilitySprites {
    // Only one instance allowed
    pub name: String,
    #[use_self_forced]
    pub prototype_base: PrototypeBaseSpec,
    pub cursor_box: UtilitySpritesCursorBox,
    pub clouds: Animation,
    pub arrow_button: Animation,
    pub explosion_chart_visualization: Animation,
    pub refresh_white: Animation,
    pub center: Sprite,
    pub check_mark: Sprite,
    pub check_mark_white: Sprite,
    pub check_mark_green: Sprite,
    pub check_mark_dark_green: Sprite,
    pub not_played_yet_green: Sprite,
    pub not_played_yet_dark_green: Sprite,
    pub played_green: Sprite,
    pub played_dark_green: Sprite,
    pub close_fat: Sprite,
    pub close_white: Sprite,
    pub close_black: Sprite,
    pub close_map_preview: Sprite,
    pub color_picker: Sprite,
    pub change_recipe: Sprite,
    pub dropdown: Sprite,
    pub downloading: Sprite,
    pub downloading_white: Sprite,
    pub downloaded: Sprite,
    pub downloaded_white: Sprite,
    pub equipment_grid: Sprite,
    pub expand_dots: Sprite,
    pub expand_dots_white: Sprite,
    pub export: Sprite,
    pub import: Sprite,
    pub map: Sprite,
    pub map_exchange_string: Sprite,
    pub missing_mod_icon: Sprite,
    pub not_available: Sprite,
    pub play: Sprite,
    pub stop: Sprite,
    pub preset: Sprite,
    pub refresh: Sprite,
    pub reset: Sprite,
    pub reset_white: Sprite,
    pub shuffle: Sprite,
    pub station_name: Sprite,
    pub search_black: Sprite,
    pub search_white: Sprite,
    pub sync_mods: Sprite,
    pub trash: Sprite,
    pub trash_white: Sprite,
    pub copy: Sprite,
    pub reassign: Sprite,
    pub warning: Sprite,
    pub warning_white: Sprite,
    pub list_view: Sprite,
    pub grid_view: Sprite,
    pub reference_point: Sprite,
    pub mouse_cursor: Sprite,
    pub mod_dependency_arrow: Sprite,
    pub add: Sprite,
    pub clone: Sprite,
    pub go_to_arrow: Sprite,
    pub pause: Sprite,
    pub speed_down: Sprite,
    pub speed_up: Sprite,
    pub editor_speed_down: Sprite,
    pub editor_pause: Sprite,
    pub editor_play: Sprite,
    pub editor_speed_up: Sprite,
    pub tick_once: Sprite,
    pub tick_sixty: Sprite,
    pub tick_custom: Sprite,
    pub search_icon: Sprite,
    pub too_far: Sprite,
    pub shoot_cursor_green: Sprite,
    pub shoot_cursor_red: Sprite,
    pub electricity_icon: Sprite,
    pub fuel_icon: Sprite,
    pub ammo_icon: Sprite,
    pub fluid_icon: Sprite,
    pub warning_icon: Sprite,
    pub danger_icon: Sprite,
    pub destroyed_icon: Sprite,
    pub recharge_icon: Sprite,
    pub too_far_from_roboport_icon: Sprite,
    pub pump_cannot_connect_icon: Sprite,
    pub not_enough_repair_packs_icon: Sprite,
    pub not_enough_construction_robots_icon: Sprite,
    pub no_building_material_icon: Sprite,
    pub no_storage_space_icon: Sprite,
    pub electricity_icon_unplugged: Sprite,
    pub game_stopped_visualization: Sprite,
    pub health_bar_green_pip: Sprite,
    pub health_bar_yellow_pip: Sprite,
    pub health_bar_red_pip: Sprite,
    pub ghost_bar_pip: Sprite,
    pub bar_gray_pip: Sprite,
    pub shield_bar_pip: Sprite,
    pub hand: Sprite,
    pub hand_black: Sprite,
    pub entity_info_dark_background: Sprite,
    pub medium_gui_arrow: Sprite,
    pub small_gui_arrow: Sprite,
    pub light_medium: Sprite,
    pub light_small: Sprite,
    pub light_cone: Sprite,
    pub color_effect: Sprite,
    pub clock: Sprite,
    pub default_ammo_damage_modifier_icon: Sprite,
    pub default_gun_speed_modifier_icon: Sprite,
    pub default_turret_attack_modifier_icon: Sprite,
    pub hint_arrow_up: Sprite,
    pub hint_arrow_down: Sprite,
    pub hint_arrow_right: Sprite,
    pub hint_arrow_left: Sprite,
    pub fluid_indication_arrow: Sprite,
    pub fluid_indication_arrow_both_ways: Sprite,
    pub heat_exchange_indication: Sprite,
    pub indication_arrow: Sprite,
    pub rail_planner_indication_arrow: Sprite,
    pub rail_planner_indication_arrow_too_far: Sprite,
    pub rail_path_not_possible: Sprite,
    pub indication_line: Sprite,
    pub short_indication_line: Sprite,
    pub short_indication_line_green: Sprite,
    pub slot_icon_module: Sprite,
    pub slot_icon_module_black: Sprite,
    pub slot_icon_armor: Sprite,
    pub slot_icon_armor_black: Sprite,
    pub slot_icon_gun: Sprite,
    pub slot_icon_gun_black: Sprite,
    pub slot_icon_ammo: Sprite,
    pub slot_icon_ammo_black: Sprite,
    pub slot_icon_resource: Sprite,
    pub slot_icon_resource_black: Sprite,
    pub slot_icon_fuel: Sprite,
    pub slot_icon_fuel_black: Sprite,
    pub slot_icon_result: Sprite,
    pub slot_icon_result_black: Sprite,
    pub slot_icon_robot: Sprite,
    pub slot_icon_robot_black: Sprite,
    pub slot_icon_robot_material: Sprite,
    pub slot_icon_robot_material_black: Sprite,
    pub slot_icon_inserter_hand: Sprite,
    pub slot_icon_inserter_hand_black: Sprite,
    pub upgrade_blueprint: Sprite,
    pub slot: Sprite,
    pub equipment_slot: Sprite,
    pub equipment_collision: Sprite,
    pub battery: Sprite,
    pub green_circle: Sprite,
    pub green_dot: Sprite,
    pub robot_slot: Sprite,
    pub set_bar_slot: Sprite,
    pub missing_icon: Sprite,
    pub deconstruction_mark: Sprite,
    pub upgrade_mark: Sprite,
    pub confirm_slot: Sprite,
    pub export_slot: Sprite,
    pub import_slot: Sprite,
    pub none_editor_icon: Sprite,
    pub cable_editor_icon: Sprite,
    pub tile_editor_icon: Sprite,
    pub decorative_editor_icon: Sprite,
    pub resource_editor_icon: Sprite,
    pub entity_editor_icon: Sprite,
    pub item_editor_icon: Sprite,
    pub force_editor_icon: Sprite,
    pub clone_editor_icon: Sprite,
    pub scripting_editor_icon: Sprite,
    pub paint_bucket_icon: Sprite,
    pub surface_editor_icon: Sprite,
    pub time_editor_icon: Sprite,
    pub cliff_editor_icon: Sprite,
    pub brush_icon: Sprite,
    pub spray_icon: Sprite,
    pub cursor_icon: Sprite,
    pub area_icon: Sprite,
    pub line_icon: Sprite,
    pub variations_tool_icon: Sprite,
    pub lua_snippet_tool_icon: Sprite,
    pub editor_selection: Sprite,
    pub brush_square_shape: Sprite,
    pub brush_circle_shape: Sprite,
    pub player_force_icon: Sprite,
    pub neutral_force_icon: Sprite,
    pub enemy_force_icon: Sprite,
    pub nature_icon: Sprite,
    pub no_nature_icon: Sprite,
    pub multiplayer_waiting_icon: Sprite,
    pub spawn_flag: Sprite,
    pub questionmark: Sprite,
    pub copper_wire: Sprite,
    pub green_wire: Sprite,
    pub red_wire: Sprite,
    pub green_wire_hightlight: Sprite,
    pub red_wire_hightlight: Sprite,
    pub wire_shadow: Sprite,
    pub and_or: Sprite,
    pub left_arrow: Sprite,
    pub right_arrow: Sprite,
    pub down_arrow: Sprite,
    pub enter: Sprite,
    pub side_menu_blueprint_library_icon: Sprite,
    pub side_menu_production_icon: Sprite,
    pub side_menu_bonus_icon: Sprite,
    pub side_menu_tutorials_icon: Sprite,
    pub side_menu_train_icon: Sprite,
    pub side_menu_achievements_icon: Sprite,
    pub side_menu_menu_icon: Sprite,
    pub side_menu_map_icon: Sprite,
    pub side_menu_blueprint_library_hover_icon: Sprite,
    pub side_menu_production_hover_icon: Sprite,
    pub side_menu_bonus_hover_icon: Sprite,
    pub side_menu_tutorials_hover_icon: Sprite,
    pub side_menu_train_hover_icon: Sprite,
    pub side_menu_achievements_hover_icon: Sprite,
    pub side_menu_menu_hover_icon: Sprite,
    pub side_menu_map_hover_icon: Sprite,
    pub circuit_network_panel_black: Sprite,
    pub circuit_network_panel_white: Sprite,
    pub logistic_network_panel_black: Sprite,
    pub logistic_network_panel_white: Sprite,
    pub rename_icon_small_black: Sprite,
    pub rename_icon_small_white: Sprite,
    pub rename_icon_normal: Sprite,
    pub achievement_label_locked: Sprite,
    pub achievement_label_unlocked_off: Sprite,
    pub achievement_label_unlocked: Sprite,
    pub achievement_label_failed: Sprite,
    pub rail_signal_placement_indicator: Sprite,
    pub train_stop_placement_indicator: Sprite,
    pub placement_indicator_leg: Sprite,
    pub grey_rail_signal_placement_indicator: Sprite,
    pub grey_placement_indicator_leg: Sprite,
    pub logistic_radius_visualization: Sprite,
    pub construction_radius_visualization: Sprite,
    pub track_button: Sprite,
    pub show_logistics_network_in_map_view: Sprite,
    pub show_electric_network_in_map_view: Sprite,
    pub show_turret_range_in_map_view: Sprite,
    pub show_pollution_in_map_view: Sprite,
    pub show_train_station_names_in_map_view: Sprite,
    pub show_player_names_in_map_view: Sprite,
    pub show_tags_in_map_view: Sprite,
    pub show_worker_robots_in_map_view: Sprite,
    pub show_rail_signal_states_in_map_view: Sprite,
    pub show_recipe_icons_in_map_view: Sprite,
    pub show_logistics_network_in_map_view_black: Sprite,
    pub show_electric_network_in_map_view_black: Sprite,
    pub show_turret_range_in_map_view_black: Sprite,
    pub show_pollution_in_map_view_black: Sprite,
    pub show_train_station_names_in_map_view_black: Sprite,
    pub show_player_names_in_map_view_black: Sprite,
    pub show_tags_in_map_view_black: Sprite,
    pub show_worker_robots_in_map_view_black: Sprite,
    pub show_rail_signal_states_in_map_view_black: Sprite,
    pub show_recipe_icons_in_map_view_black: Sprite,
    pub train_stop_in_map_view: Sprite,
    pub train_stop_disabled_in_map_view: Sprite,
    pub train_stop_full_in_map_view: Sprite,
    pub custom_tag_in_map_view: Sprite,
    pub covered_chunk: Sprite,
    pub white_square: Sprite,
    pub white_mask: Sprite,
    pub favourite_server_icon: Sprite,
    pub crafting_machine_recipe_not_unlocked: Sprite,
    pub gps_map_icon: Sprite,
    pub custom_tag_icon: Sprite,
    pub underground_remove_belts: Sprite,
    pub underground_remove_pipes: Sprite,
    pub underground_pipe_connection: Sprite,
    pub ghost_cursor: Sprite,
    pub tile_ghost_cursor: Sprite,
    pub expand: Sprite,
    pub expand_dark: Sprite,
    pub collapse: Sprite,
    pub collapse_dark: Sprite,
    pub status_working: Sprite,
    pub status_not_working: Sprite,
    pub status_yellow: Sprite,
    pub gradient: Sprite,
    pub output_console_gradient: Sprite,
    pub select_icon_black: Sprite,
    pub select_icon_white: Sprite,
    pub notification: Sprite,
    pub alert_arrow: Sprite,
    pub technology_black: Sprite,
    pub technology_white: Sprite,
    pub inserter_stack_size_bonus_modifier_icon: Sprite,
    pub inserter_stack_size_bonus_modifier_constant: Option<Sprite>,
    pub stack_inserter_capacity_bonus_modifier_icon: Sprite,
    pub stack_inserter_capacity_bonus_modifier_constant: Option<Sprite>,
    pub laboratory_speed_modifier_icon: Sprite,
    pub laboratory_speed_modifier_constant: Option<Sprite>,
    pub character_logistic_slots_modifier_icon: Sprite,
    pub character_logistic_slots_modifier_constant: Option<Sprite>,
    pub character_logistic_trash_slots_modifier_icon: Sprite,
    pub character_logistic_trash_slots_modifier_constant: Option<Sprite>,
    pub maximum_following_robots_count_modifier_icon: Sprite,
    pub maximum_following_robots_count_modifier_constant: Option<Sprite>,
    pub worker_robot_speed_modifier_icon: Sprite,
    pub worker_robot_speed_modifier_constant: Option<Sprite>,
    pub worker_robot_storage_modifier_icon: Sprite,
    pub worker_robot_storage_modifier_constant: Option<Sprite>,
    pub ghost_time_to_live_modifier_icon: Sprite,
    pub ghost_time_to_live_modifier_constant: Option<Sprite>,
    pub turret_attack_modifier_icon: Sprite,
    pub turret_attack_modifier_constant: Option<Sprite>,
    pub ammo_damage_modifier_icon: Sprite,
    pub ammo_damage_modifier_constant: Option<Sprite>,
    pub give_item_modifier_icon: Sprite,
    pub give_item_modifier_constant: Option<Sprite>,
    pub gun_speed_modifier_icon: Sprite,
    pub gun_speed_modifier_constant: Option<Sprite>,
    pub unlock_recipe_modifier_icon: Sprite,
    pub unlock_recipe_modifier_constant: Option<Sprite>,
    pub character_crafting_speed_modifier_icon: Sprite,
    pub character_crafting_speed_modifier_constant: Option<Sprite>,
    pub character_mining_speed_modifier_icon: Sprite,
    pub character_mining_speed_modifier_constant: Option<Sprite>,
    pub character_running_speed_modifier_icon: Sprite,
    pub character_running_speed_modifier_constant: Option<Sprite>,
    pub character_build_distance_modifier_icon: Sprite,
    pub character_build_distance_modifier_constant: Option<Sprite>,
    pub character_item_drop_distance_modifier_icon: Sprite,
    pub character_item_drop_distance_modifier_constant: Option<Sprite>,
    pub character_reach_distance_modifier_icon: Sprite,
    pub character_reach_distance_modifier_constant: Option<Sprite>,
    pub character_resource_reach_distance_modifier_icon: Sprite,
    pub character_resource_reach_distance_modifier_constant: Option<Sprite>,
    pub character_item_pickup_distance_modifier_icon: Sprite,
    pub character_item_pickup_distance_modifier_constant: Option<Sprite>,
    pub character_loot_pickup_distance_modifier_icon: Sprite,
    pub character_loot_pickup_distance_modifier_constant: Option<Sprite>,
    pub character_inventory_slots_bonus_modifier_icon: Sprite,
    pub character_inventory_slots_bonus_modifier_constant: Option<Sprite>,
    pub deconstruction_time_to_live_modifier_icon: Sprite,
    pub deconstruction_time_to_live_modifier_constant: Option<Sprite>,
    pub max_failed_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,
    pub max_failed_attempts_per_tick_per_construction_queue_modifier_constant: Option<Sprite>,
    pub max_successful_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,
    pub max_successful_attempts_per_tick_per_construction_queue_modifier_constant: Option<Sprite>,
    pub character_health_bonus_modifier_icon: Sprite,
    pub character_health_bonus_modifier_constant: Option<Sprite>,
    pub mining_drill_productivity_bonus_modifier_icon: Sprite,
    pub mining_drill_productivity_bonus_modifier_constant: Option<Sprite>,
    pub train_braking_force_bonus_modifier_icon: Sprite,
    pub train_braking_force_bonus_modifier_constant: Option<Sprite>,
    pub zoom_to_world_enabled_modifier_icon: Sprite,
    pub zoom_to_world_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_ghost_building_enabled_modifier_icon: Sprite,
    pub zoom_to_world_ghost_building_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_blueprint_enabled_modifier_icon: Sprite,
    pub zoom_to_world_blueprint_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_deconstruction_planner_enabled_modifier_icon: Sprite,
    pub zoom_to_world_deconstruction_planner_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_upgrade_planner_enabled_modifier_icon: Sprite,
    pub zoom_to_world_upgrade_planner_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_selection_tool_enabled_modifier_icon: Sprite,
    pub zoom_to_world_selection_tool_enabled_modifier_constant: Option<Sprite>,
    pub worker_robot_battery_modifier_icon: Sprite,
    pub worker_robot_battery_modifier_constant: Option<Sprite>,
    pub laboratory_productivity_modifier_icon: Sprite,
    pub laboratory_productivity_modifier_constant: Option<Sprite>,
    pub follower_robot_lifetime_modifier_icon: Sprite,
    pub follower_robot_lifetime_modifier_constant: Option<Sprite>,
    pub artillery_range_modifier_icon: Sprite,
    pub artillery_range_modifier_constant: Option<Sprite>,
    pub nothing_modifier_icon: Sprite,
    pub nothing_modifier_constant: Option<Sprite>,
    pub character_additional_mining_categories_modifier_icon: Sprite,
    pub character_additional_mining_categories_modifier_constant: Option<Sprite>,
    pub character_logistic_requests_modifier_icon: Sprite,
    pub character_logistic_requests_modifier_constant: Option<Sprite>,
}

#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UtilitySpritesCursorBox {
    pub regular: Vec<BoxSpecification>,
    pub not_allowed: Vec<BoxSpecification>,
    pub copy: Vec<BoxSpecification>,
    pub electricity: Vec<BoxSpecification>,
    pub logistics: Vec<BoxSpecification>,
    pub pair: Vec<BoxSpecification>,
    pub train_visualization: Vec<BoxSpecification>,
    pub blueprint_snap_rectangle: Vec<BoxSpecification>,
}
