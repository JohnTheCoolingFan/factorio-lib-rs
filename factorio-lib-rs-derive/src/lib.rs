//! This crate is not intended for use outside of [factorio-lib-rs]!

extern crate proc_macro;

use core::fmt::Display;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Attribute, Ident, Result};
use syn::spanned::Spanned;
use core::iter::Iterator;

#[proc_macro_derive(Prototype)]
pub fn prototype_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_prototype_macro(&ast)
}

#[proc_macro_derive(ModSetting)]
pub fn mod_setting_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mod_setting_macro(&ast)
}

#[proc_macro_derive(PrototypeBase)]
pub fn prototype_base_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_prototype_base_macro(&ast)
}

#[proc_macro_derive(TriggerEffectItemBase)]
pub fn trigger_effect_item_base_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_trigger_efffect_item_base_macro(&ast)
}

#[proc_macro_derive(CreateEntityTriggerEffectItemBase)]
pub fn create_entity_trigger_effect_item_base_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_create_entity_trigger_effect_item_base_macro(&ast)
}

#[proc_macro_derive(TriggerItemBase)]
pub fn trigger_item_base_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_trigger_item_base_macro(&ast)
}

#[proc_macro_derive(Entity)]
pub fn entity_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_entity_macro(&ast);
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(Corpse)]
pub fn corpse_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_corpse_macro(&ast);
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(EntityWithHealth)]
pub fn entity_with_health_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_entity_with_health_macro(&ast);
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(EntityWithOwner)]
pub fn entity_with_owner_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_entity_with_owner_macro(&ast);
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(Combinator)]
pub fn combinator_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_combinator_macro(&ast);
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(CraftingMachine)]
pub fn crafting_machine_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_crafting_machine_macro(&ast);
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(FlyingRobot)]
pub fn flying_robot_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_flying_robot_macro(&ast);
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(TransportBeltConnectable)]
pub fn transport_belt_connectable_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_transport_belt_connectable_macro(&ast);
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(Turret)]
pub fn turret_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_turret_macro(&ast);
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

#[proc_macro_derive(Vehicle)]
pub fn vehicle_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_vehicle_macro(&ast);
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

/// <https://wiki.factorio.com/Prototype/RollingStock>
#[proc_macro_derive(RollingStock)]
pub fn rolling_stock_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_rolling_stock_macro(&ast);
    ts.extend(impl_vehicle_macro(&ast));
    ts.extend(impl_entity_with_owner_macro(&ast));
    ts.extend(impl_entity_with_health_macro(&ast));
    ts.extend(impl_entity_macro(&ast));
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

/// <https://wiki.factorio.com/Prototype/Equipment>
#[proc_macro_derive(Equipment)]
pub fn equipment_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let mut ts = impl_equipment_macro(&ast);
    ts.extend(impl_prototype_base_macro(&ast));
    ts
}

/// <https://wiki.factorio.com/Prototype/Item>
#[proc_macro_derive(Item)]
pub fn item_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_item_macro(&ast)
}

/// <https://wiki.factorio.com/Prototype/SelectionTool>
#[proc_macro_derive(SelectionTool)]
pub fn selection_tool_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_selection_tool_macro(&ast)
}

#[proc_macro_derive(DataTableAccessable, attributes(data_table))]
pub fn data_table_accessable_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_data_table_accessable_macro(&ast)
}

fn impl_prototype_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Prototype for #name {
            fn name(&self) -> &String { &self.name }
        }
    };
    gen.into()
}

fn impl_mod_setting_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ModSetting for #name {
            fn localised_name(&self) -> &Option<LocalisedString> { &self.localised_name }
            fn localised_description(&self) -> &Option<LocalisedString> { &self.localised_description }
            fn order(&self) -> &Option<String> { &self.order }
            fn hidden(&self) -> bool { self.hidden }
            fn setting_type(&self) -> ModSettingType { self.setting_type }
        }
    };
    gen.into()
}

fn impl_prototype_base_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PrototypeBase for #name {
            fn localised_name(&self) -> &Option<LocalisedString> { &self.prototype_base.localised_name }
            fn localised_description(&self) -> &Option<LocalisedString> { &self.prototype_base.localised_description }
            fn order(&self) -> &String { &self.prototype_base.order }
        }
    };
    gen.into()
}

fn impl_trigger_efffect_item_base_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TriggerEffectItemBase for #name {
            fn repeat_count(&self) -> u16 { self.base.repeat_count }
            fn repeat_count_deviation(&self) -> u16 { self.base.repeat_count_deviation }
            fn probability(&self) -> f32 { self.base.probability }
            fn affects_target(&self) -> bool { self.base.affects_target }
            fn show_in_tooltip(&self) -> bool { self.base.show_in_tooltip }
            fn damage_type_filters(&self) -> &Option<DamageTypeFilters> { &self.base.damage_type_filters }
        }
    };
    gen.into()
}

fn impl_create_entity_trigger_effect_item_base_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CreateEntityTriggerEffectItemBase for #name {
            fn entity_name(&self) -> &String { &self.create_entity_base.entity_name }
            fn offset_deviation(&self) -> &Option<BoundingBox> { &self.create_entity_base.offset_deviation }
            fn trigger_created_entity(&self) -> bool { self.create_entity_base.trigger_created_entity }
            fn check_buildability(&self) -> bool { self.create_entity_base.check_buildability }
            fn show_in_tooltip(&self) -> bool { self.create_entity_base.show_in_tooltip }
            fn tile_collision_mask(&self) -> &Option<CollisionMask> { &self.create_entity_base.tile_collision_mask }
            fn offsets(&self) -> &Option<Vec<Factorio2DVector>> { &self.create_entity_base.offsets }
        }
    };
    gen.into()
}

fn impl_trigger_item_base_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TriggerItemBase for #name {
            fn entity_flags(&self) -> EntityPrototypeFlags { self.base.entity_flags }
            fn ignore_collision_condition(&self) -> bool { self.base.ignore_collision_condition }
            fn trigger_target_mask(&self) -> &TriggerTargetMask { &self.base.trigger_target_mask }
            fn repeat_count(&self) -> u32 { self.base.repeat_count }
            fn probability(&self) -> f32 { self.base.probability }
            fn collision_mask(&self) -> CollisionMask { self.base.collision_mask }
            fn action_delivery(&self) -> &Option<Vec<TriggerDelivery>> { &self.base.action_delivery }
            fn force(&self) -> ForceCondition { self.base.force }
        }
    };
    gen.into()
}

fn impl_entity_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Entity for #name {
            fn icon(&self) -> &Option<IconSpecification> { &self.entity_base.icon }
            fn collision_box(&self) -> BoundingBox { self.entity_base.collision_box }
            fn collision_mask(&self) -> CollisionMask { self.entity_base.collision_mask }
            fn map_generator_bounding_box(&self) -> BoundingBox { self.entity_base.map_generator_bounding_box }
            fn selection_box(&self) -> BoundingBox { self.entity_base.selection_box }
            fn drawing_box(&self) -> BoundingBox { self.entity_base.drawing_box }
            fn sticker_box(&self) -> BoundingBox { self.entity_base.sticker_box }
            fn hit_visualization_box(&self) -> BoundingBox { self.entity_base.hit_visualization_box }
            fn trigger_target_mask(&self) -> &Option<TriggerTargetMask> { &self.entity_base.trigger_target_mask }
            fn flags(&self) -> Option<EntityPrototypeFlags> { self.entity_base.flags }
            fn minable(&self) -> &MinableProperties { &self.entity_base.minable }
            fn subgroup(&self) -> &Option<String> { &self.entity_base.subgroup }
            fn allow_copy_paste(&self) -> bool { self.entity_base.allow_copy_paste }
            fn selectable_in_game(&self) -> bool { self.entity_base.selectable_in_game }
            fn selection_priority(&self) -> u8 { self.entity_base.selection_priority }
            fn remove_decoratives(&self) -> RemoveDecoratives { self.entity_base.remove_decoratives }
            fn emissions_per_second(&self) -> f64 { self.entity_base.emissions_per_second }
            fn shooting_cursor_size(&self) -> Option<f64> { self.entity_base.shooting_cursor_size }
            fn created_smoke(&self) -> &CreateTrivialSmokeEffectItem { &self.entity_base.created_smoke }
            fn working_sound(&self) -> &Option<WorkingSound> { &self.entity_base.working_sound }
            fn created_effect(&self) -> &Option<Trigger> { &self.entity_base.created_effect }
            fn build_sound(&self) -> &Option<Sound> { &self.entity_base.build_sound }
            fn mined_sound(&self) -> &Option<Sound> { &self.entity_base.mined_sound }
            fn mining_sound(&self) -> &Option<Sound> { &self.entity_base.mining_sound }
            fn rotated_sound(&self) -> &Option<Sound> { &self.entity_base.rotated_sound }
            fn vehicle_impact_sound(&self) -> &Option<Sound> { &self.entity_base.vehicle_impact_sound }
            fn open_sound(&self) -> &Option<Sound> { &self.entity_base.open_sound }
            fn close_sound(&self) -> &Option<Sound> { &self.entity_base.close_sound }
            fn radius_visualization_specification(&self) -> &Option<RadiusVisualizationSpecification> { &self.entity_base.radius_visualization_specification }
            fn build_base_evolution_requirement(&self) -> f64 { self.entity_base.build_base_evolution_requirement }
            fn alert_icon_shift(&self) -> Option<Factorio2DVector> { self.entity_base.alert_icon_shift }
            fn alert_icon_scale(&self) -> Option<f32> { self.entity_base.alert_icon_scale }
            fn fast_replaceable_group(&self) -> &String { &self.entity_base.fast_replaceable_group }
            fn next_upgrade(&self) -> &Option<String> { &self.entity_base.next_upgrade }
            fn placeable_by(&self) -> &Option<Vec<ItemToPlace>> { &self.entity_base.placeable_by }
            fn remains_when_mined(&self) -> &Option<Vec<String>> { &self.entity_base.remains_when_mined }
            fn additional_pastable_entities(&self) -> &Option<Vec<String>> { &self.entity_base.additional_pastable_entities }
            fn tile_width(&self) -> u32 { self.entity_base.tile_width }
            fn tile_height(&self) -> u32 { self.entity_base.tile_height }
            fn autoplace(&self) -> &Option<AutoplaceSpecification> { &self.entity_base.autoplace }
            fn map_color(&self) -> Option<Color> { self.entity_base.map_color }
            fn friendly_map_color(&self) -> Option<Color> { self.entity_base.friendly_map_color }
            fn enemy_map_color(&self) -> Option<Color> { self.entity_base.enemy_map_color }
            fn water_reflection(&self) -> &Option<WaterReflectionDefinition> { &self.entity_base.water_reflection }
            fn protected_from_tile_building(&self) -> bool { self.entity_base.protected_from_tile_building }
        }
    };
    gen.into()
}

fn impl_corpse_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl  Corpse for #name {
            fn dying_speed(&self) -> f32 { self.corpse_base.dying_speed }
            fn splash_speed(&self) -> f32 { self.corpse_base.splash_speed }
            fn time_before_shading_off(&self) -> i32 { self.corpse_base.time_before_shading_off }
            fn time_before_removed(&self) -> i32 { self.corpse_base.time_before_removed }
            fn remove_on_entity_placemen(&self) -> bool { self.corpse_base.remove_on_entity_placemen }
            fn remove_on_tile_placement(&self) -> bool { self.corpse_base.remove_on_tile_placement }
            fn final_render_layer(&self) -> RenderLayer { self.corpse_base.final_render_layer }
            fn gound_patch_render_layer(&self) -> RenderLayer { self.corpse_base.gound_patch_render_layer }
            fn animation_render_layer(&self) -> RenderLayer { self.corpse_base.animation_render_layer }
            fn splash_render_layer(&self) -> RenderLayer { self.corpse_base.splash_render_layer }
            fn animation_overlay_render_layer(&self) -> RenderLayer { self.corpse_base.animation_overlay_render_layer }
            fn animation_overlay_final_render_layer(&self) -> RenderLayer { self.corpse_base.animation_overlay_final_render_layer }
            fn shuffle_directions_at_frame(&self) -> u8 { self.corpse_base.shuffle_directions_at_frame }
            fn use_tile_color_for_ground_patch_tint(&self) -> bool { self.corpse_base.use_tile_color_for_ground_patch_tint }
            fn ground_patch_fade_in_delay(&self) -> f32 { self.corpse_base.ground_patch_fade_in_delay }
            fn ground_patch_fade_in_speed(&self) -> f32 { self.corpse_base.ground_patch_fade_in_speed }
            fn ground_patch_fade_out_start(&self) -> f32 { self.corpse_base.ground_patch_fade_out_start }
            fn animation(&self) -> &Option<Vec<RotatedAnimationVariation>> { &self.corpse_base.animation }
            fn animation_overlay(&self) -> &Option<Vec<RotatedAnimationVariation>> { &self.corpse_base.animation_overlay }
            fn splash(&self) -> &Option<Vec<AnimationVariation>> { &self.corpse_base.splash }
            fn ground_patch(&self) -> &Option<Vec<AnimationVariation>> { &self.corpse_base.ground_patch }
            fn ground_patch_higher(&self) -> &Option<Vec<AnimationVariation>> { &self.corpse_base.ground_patch_higher }
            fn ground_patch_fade_out_duration(&self) -> f32 { self.corpse_base.ground_patch_fade_out_duration }
            fn direction_shuffle(&self) -> &Option<Vec<Vec<u16>>> { &self.corpse_base.direction_shuffle }
        }
    };
    gen.into()
}

fn impl_entity_with_health_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EntityWithHealth for #name {
            fn max_health(&self) -> f32 { self.entity_with_health_base.max_health }
            fn healing_per_tick(&self) -> f32 { self.entity_with_health_base.healing_per_tick }
            fn repair_speed_multiplier(&self) -> f32 { self.entity_with_health_base.repair_speed_multiplier }
            fn dying_explosion(&self) -> &Option<Vec<ExplosionDefinition>> { &self.entity_with_health_base.dying_explosion }
            fn drying_trigger_effect(&self) -> &Option<TriggerEffect> { &self.entity_with_health_base.drying_trigger_effect }
            fn damaged_trigger_effect(&self) -> &Option<TriggerEffect> { &self.entity_with_health_base.damaged_trigger_effect }
            fn loot(&self) -> &Option<Vec<Loot>> { &self.entity_with_health_base.loot }
            fn resistances(&self) -> &Option<Vec<Resistance>> { &self.entity_with_health_base.resistances }
            fn attack_reaction(&self) -> &Vec<AttackReactionItem> { &self.entity_with_health_base.attack_reaction }
            fn repair_sound(&self) -> &Sound { &self.entity_with_health_base.repair_sound }
            fn alert_when_damaged(&self) -> bool { self.entity_with_health_base.alert_when_damaged }
            fn hide_resistances(&self) -> bool { self.entity_with_health_base.hide_resistances }
            fn create_ghost_on_death(&self) -> bool { self.entity_with_health_base.create_ghost_on_death }
            fn random_corpse_variation(&self) -> bool { self.entity_with_health_base.random_corpse_variation }
            fn integration_patch_render_layer(&self) -> RenderLayer { self.entity_with_health_base.integration_patch_render_layer }
            fn corpse(&self) -> &Vec<String> { &self.entity_with_health_base.corpse }
            fn integration_patch(&self) -> &Sprite4Way { &self.entity_with_health_base.integration_patch }
        }
    };
    gen.into()
}

fn impl_entity_with_owner_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EntityWithOwner for #name {
            fn is_military_target(&self) -> bool { self.entity_with_owner_base.is_military_target }
            fn allow_run_time_change_of_is_military_target(&self) -> bool { self.entity_with_owner_base.allow_run_time_change_of_is_military_target }
        }
    };
    gen.into()
}

fn impl_combinator_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Combinator for #name {
            fn energy_source(&self) -> &EnergySource { &self.combinator_base.energy_source }
            fn active_energy_usage(&self) -> Energy { self.combinator_base.active_energy_usage }
            fn sprites(&self) -> &Sprite4Way { &self.combinator_base.sprites }
            fn activity_led_sprites(&self) -> &Sprite4Way { &self.combinator_base.activity_led_sprites }
            fn input_connection_bounding_box(&self) -> BoundingBox { self.combinator_base.input_connection_bounding_box }
            fn output_connection_bounding_box(&self) -> BoundingBox { self.combinator_base.output_connection_bounding_box }
            fn activity_led_light_offsets(&self) -> [Factorio2DVector; 4] { self.combinator_base.activity_led_light_offsets }
            fn screen_light_offsets(&self) -> [Factorio2DVector; 4] { self.combinator_base.screen_light_offsets }
            fn input_connection_points(&self) -> &[WireConnectionPoint; 4] { &self.combinator_base.input_connection_points }
            fn output_connection_points(&self) -> &[WireConnectionPoint; 4] { &self.combinator_base.output_connection_points }
            fn activity_led_light(&self) -> &Option<LightDefinition> { &self.combinator_base.activity_led_light }
            fn screen_light(&self) -> &Option<LightDefinition> { &self.combinator_base.screen_light }
            fn activity_led_hold_time(&self) -> u8 { self.combinator_base.activity_led_hold_time }
            fn circuit_wire_max_distance(&self) -> f64 { self.combinator_base.circuit_wire_max_distance }
            fn draw_copper_wires(&self) -> bool { self.combinator_base.draw_copper_wires }
            fn draw_circuit_wires(&self) -> bool { self.combinator_base.draw_circuit_wires }
        }
    };
    gen.into()
}

fn impl_crafting_machine_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CraftingMachine for #name {
            fn energy_usage(&self) -> Energy { self.crafting_machine_base.energy_usage }
            fn crafting_speed(&self) -> f64 { self.crafting_machine_base.crafting_speed }
            fn crafting_categories(&self) -> &Vec<String>{ &self.crafting_machine_base.crafting_categories }
            fn energy_source(&self) -> &EnergySource { &self.crafting_machine_base.energy_source }
            fn fluid_boxes(&self) -> &Option<Vec<FluidBox>> { &self.crafting_machine_base.fluid_boxes }
            fn allowed_effects(&self) -> &Option<EffectTypeLimitation> { &self.crafting_machine_base.allowed_effects }
            fn scale_entity_info_icon(&self) -> bool { self.crafting_machine_base.scale_entity_info_icon }
            fn show_recipe_icon(&self) -> bool { self.crafting_machine_base.show_recipe_icon }
            fn return_ingredients_on_change(&self) -> bool { self.crafting_machine_base.return_ingredients_on_change }
            fn animation(&self) -> &Option<Animation4Way> { &self.crafting_machine_base.animation }
            fn idle_animation(&self) -> &Option<Animation4Way> { &self.crafting_machine_base.idle_animation }
            fn always_draw_idle_animation(&self) -> bool { self.crafting_machine_base.always_draw_idle_animation }
            fn default_recipe_tint(&self) -> &Option<RecipeTint> { &self.crafting_machine_base.default_recipe_tint }
            fn shift_animation_waypoints(&self) -> &Option<ShiftAnimationWaypoints> { &self.crafting_machine_base.shift_animation_waypoints }
            fn shift_animation_waypoint_stop_duration(&self) -> u16 { self.crafting_machine_base.shift_animation_waypoint_stop_duration }
            fn shift_animation_transition_duration(&self) -> u16 { self.crafting_machine_base.shift_animation_transition_duration }
            fn status_colors(&self) -> &Option<StatusColors> { &self.crafting_machine_base.status_colors }
            fn entity_info_icon_shift(&self) -> Factorio2DVector { self.crafting_machine_base.entity_info_icon_shift }
            fn draw_entity_info_icon_background(&self) -> bool { self.crafting_machine_base.draw_entity_info_icon_background }
            fn match_animation_speed_to_activity(&self) -> bool { self.crafting_machine_base.match_animation_speed_to_activity }
            fn show_recipe_icon_on_map(&self) -> bool { self.crafting_machine_base.show_recipe_icon_on_map }
            fn base_productivity(&self) -> f32 { self.crafting_machine_base.base_productivity }
            fn module_specification(&self) -> &Option<ModuleSpecification> { &self.crafting_machine_base.module_specification }
            fn working_visualisations(&self) -> &Option<Vec<WorkingVisualisation>> { &self.crafting_machine_base.working_visualisations }
        }
    };
    gen.into()
}

fn impl_flying_robot_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl FlyingRobot for #name {
            fn speed(&self) -> f64 { self.flying_robot_base.speed }
            fn max_speed(&self) -> f64 { self.flying_robot_base.max_speed }
            fn max_energy(&self) -> Energy { self.flying_robot_base.max_energy }
            fn energy_per_move(&self) -> Energy { self.flying_robot_base.energy_per_move }
            fn energy_per_tick(&self) -> Energy { self.flying_robot_base.energy_per_tick }
            fn min_to_charge(&self) -> f32 { self.flying_robot_base.min_to_charge }
            fn max_to_charge(&self) -> f32 { self.flying_robot_base.max_to_charge }
            fn speed_multiplier_when_out_of_energy(&self) -> f32 { self.flying_robot_base.speed_multiplier_when_out_of_energy }
        }
    };
    gen.into()
}

fn impl_transport_belt_connectable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TransportBeltConnectable for #name {
            fn speed(&self) -> f64 { self.transport_belt_connectable_base.speed }
            fn animation_speed_coefficient(&self) -> f64 { self.transport_belt_connectable_base.animation_speed_coefficient }
            fn belt_animation_set(&self) -> &TransportBeltConnectableGraphics { &self.transport_belt_connectable_base.belt_animation_set }
        }
    };
    gen.into()
}

fn impl_turret_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Turret for #name {
            fn attack_parameters(&self) -> &AttackParameters { &self.turret_base.attack_parameters }
            fn folded_animation(&self) -> &RotatedAnimation4Way { &self.turret_base.folded_animation }
            fn call_for_help_radius(&self) -> f64 { self.turret_base.call_for_help_radius }
            fn corpse(&self) -> &Option<String>  { &self.turret_base.corpse }
            fn attack_target_mask(&self) -> &Option<TriggerTargetMask>  { &self.turret_base.attack_target_mask }
            fn ignore_target_mask(&self) -> &Option<TriggerTargetMask>  { &self.turret_base.ignore_target_mask }
            fn shoot_in_prepare_state(&self) -> bool  { self.turret_base.shoot_in_prepare_state }
            fn turret_base_has_direction(&self) -> bool  { self.turret_base.turret_base_has_direction }
            fn random_animation_offset(&self) -> bool  { self.turret_base.random_animation_offset }
            fn secondary_animation(&self) -> bool  { self.turret_base.secondary_animation }
            fn attack_from_start_frame(&self) -> bool  { self.turret_base.attack_from_start_frame }
            fn allow_turning_when_starting_attack(&self) -> bool  { self.turret_base.allow_turning_when_starting_attack }
            fn base_picture_secondary_draw_order(&self) -> u8  { self.turret_base.base_picture_secondary_draw_order }
            fn gun_animation_secondary_draw_order(&self) -> u8  { self.turret_base.gun_animation_secondary_draw_order }
            fn base_picture_render_layer(&self) -> RenderLayer  { self.turret_base.base_picture_render_layer }
            fn gun_animation_render_layer(&self) -> RenderLayer  { self.turret_base.gun_animation_render_layer }
            fn base_picture(&self) -> &Option<Animation4Way> { &self.turret_base.base_picture }
            fn preparing_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.preparing_animation }
            fn prepared_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.prepared_animation }
            fn prepared_alternative_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.prepared_alternative_animation }
            fn starting_attack_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.starting_attack_animation }
            fn attacking_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.attacking_animation }
            fn energy_glow_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.energy_glow_animation }
            fn ending_attack_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.ending_attack_animation }
            fn folding_animation(&self) -> &Option<RotatedAnimation4Way> { &self.turret_base.folding_animation }
            fn integration(&self) -> &Option<Sprite> { &self.turret_base.integration }
            fn glow_light_intensity(&self) -> f32  { self.turret_base.glow_light_intensity }
            fn starting_attack_sound(&self) -> &Option<Sound> { &self.turret_base.starting_attack_sound }
            fn dying_sound(&self) -> &Option<Sound> { &self.turret_base.dying_sound }
            fn preparing_sound(&self) -> &Option<Sound> { &self.turret_base.preparing_sound }
            fn folding_sound(&self) -> &Option<Sound> { &self.turret_base.folding_sound }
            fn prepared_sound(&self) -> &Option<Sound> { &self.turret_base.prepared_sound }
            fn prepared_alternative_sound(&self) -> &Option<Sound> { &self.turret_base.prepared_alternative_sound }
            fn rotation_speed(&self) -> f32  { self.turret_base.rotation_speed }
            fn preparing_speed(&self) -> f32  { self.turret_base.preparing_speed }
            fn folded_speed(&self) -> f32  { self.turret_base.folded_speed }
            fn folded_speed_secondary(&self) -> f32  { self.turret_base.folded_speed_secondary }
            fn prepared_speed(&self) -> f32  { self.turret_base.prepared_speed }
            fn prepared_speed_secondary(&self) -> f32  { self.turret_base.prepared_speed_secondary }
            fn prepared_alternative_speed(&self) -> f32  { self.turret_base.prepared_alternative_speed }
            fn prepared_alternative_speed_secondary(&self) -> f32  { self.turret_base.prepared_alternative_speed_secondary }
            fn prepared_alternative_chance(&self) -> f32  { self.turret_base.prepared_alternative_chance }
            fn starting_attack_speed(&self) -> f32  { self.turret_base.starting_attack_speed }
            fn attacking_speed(&self) -> f32  { self.turret_base.attacking_speed }
            fn ending_attack_speed(&self) -> f32  { self.turret_base.ending_attack_speed }
            fn folding_speed(&self) -> f32  { self.turret_base.folding_speed }
            fn prepare_range(&self) -> f64  { self.turret_base.prepare_range }
            fn alert_when_attacking(&self) -> bool  { self.turret_base.alert_when_attacking }
            fn spawn_decorations_on_expansion(&self) -> bool  { self.turret_base.spawn_decorations_on_expansion }
            fn spawn_decoration(&self) -> &Option<Vec<CreateDecorativesTriggerEffectItem>> { &self.turret_base.spawn_decoration }
        }
    };
    gen.into()
}

fn impl_vehicle_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Vehicle for #name {
            fn weight(&self) -> f64 { self.vehicle_base.weight }
            fn braking_force(&self) -> f64 { self.vehicle_base.braking_force }
            fn friction_force(&self) -> f64 { self.vehicle_base.friction_force }
            fn energy_per_hit_point(&self) -> f64 { self.vehicle_base.energy_per_hit_point }
            fn terrain_friction_modifier(&self) -> f32 { self.vehicle_base.terrain_friction_modifier }
            fn sound_minimum_speed(&self) -> f64 { self.vehicle_base.sound_minimum_speed }
            fn sound_scaling_ratio(&self) -> f64 { self.vehicle_base.sound_scaling_ratio }
            fn stop_trigger_speed(&self) -> f64 { self.vehicle_base.stop_trigger_speed }
            fn crash_trigger(&self) -> &Option<TriggerEffect> { &self.vehicle_base.crash_trigger }
            fn stop_trigger(&self) -> &Option<TriggerEffect> { &self.vehicle_base.stop_trigger }
            fn equipment_grid(&self) -> &Option<String> { &self.vehicle_base.equipment_grid }
            fn minimap_representation(&self) -> &Option<Sprite> { &self.vehicle_base.minimap_representation }
            fn selected_minimap_representation(&self) -> &Option<Sprite> { &self.vehicle_base.selected_minimap_representation }
            fn allow_passengers(&self) -> bool { self.vehicle_base.allow_passengers }
        }
    };
    gen.into()
}

fn impl_rolling_stock_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl RollingStock for #name {
            fn max_speed(&self) -> f64 { self.rolling_stock_base.max_speed }
            fn air_resistance(&self) -> f64 { self.rolling_stock_base.air_resistance }
            fn joint_distance(&self) -> f64 { self.rolling_stock_base.joint_distance }
            fn connection_distance(&self) -> f64 { self.rolling_stock_base.connection_distance }
            fn pictures(&self) -> &RotatedSprite { &self.rolling_stock_base.pictures }
            fn vertical_selection_shift(&self) -> f64 { self.rolling_stock_base.vertical_selection_shift }
            fn drive_over_tie_trigger(&self) -> &Option<TriggerEffect> { &self.rolling_stock_base.drive_over_tie_trigger }
            fn tie_distance(&self) -> f64 { self.rolling_stock_base.tie_distance }
            fn back_light(&self) -> &Option<LightDefinition> { &self.rolling_stock_base.back_light }
            fn stand_by_light(&self) -> &Option<LightDefinition> { &self.rolling_stock_base.stand_by_light }
            fn wheels(&self) -> &Option<RotatedSprite> { &self.rolling_stock_base.wheels }
            fn horizontal_doors(&self) -> &Option<Animation> { &self.rolling_stock_base.horizontal_doors }
            fn vertical_doors(&self) -> &Option<Animation> { &self.rolling_stock_base.vertical_doors }
            fn color(&self) -> &Option<Color> { &self.rolling_stock_base.color }
            fn allow_manual_color(&self) -> bool { self.rolling_stock_base.allow_manual_color }
            fn allow_robot_dispatch_in_automatic_mode(&self) -> bool { self.rolling_stock_base.allow_robot_dispatch_in_automatic_mode }
        }
    };
    gen.into()
}

fn impl_equipment_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Equipment for #name {
            fn sprite(&self) -> &Sprite { &self.equipment_base.sprite }
            fn shape(&self) -> &EquipmentShape { &self.equipment_base.shape }
            fn categories(&self) -> &Vec<String> { &self.equipment_base.categories }
            fn energy_source(&self) -> &EnergySource { &self.equipment_base.energy_source }
            fn take_result(&self) -> &String { &self.equipment_base.take_result }
            fn background_color(&self) -> &Color { &self.equipment_base.background_color }
            fn background_border_color(&self) -> &Color { &self.equipment_base.background_border_color }
            fn grabbed_background_color(&self) -> &Color { &self.equipment_base.grabbed_background_color }
        }
    };
    gen.into()
}

fn impl_item_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Item for #name {
            fn icon(&self) -> &IconSpecification { &self.item_base.icon }
            fn stack_size(&self) -> u32 { self.item_base.stack_size }
            fn place_result(&self) -> &String { &self.item_base.place_result }
            fn placed_as_equipment_result(&self) -> &String { &self.item_base.placed_as_equipment_result }
            fn subgroup(&self) -> &String { &self.item_base.subgroup }
            fn fuel_category(&self) -> &String { &self.item_base.fuel_category }
            fn burnt_result(&self) -> &String { &self.item_base.burnt_result }
            fn place_as_tile(&self) -> &Option<PlaceAsTile> { &self.item_base.place_as_tile }
            fn pictures(&self) -> &Option<SpriteVariations> { &self.item_base.pictures }
            fn flags(&self) -> &Option<ItemPrototypeFlags> { &self.item_base.flags }
            fn default_request_amount(&self) -> u32 { self.item_base.default_request_amount }
            fn wire_count(&self) -> u32 { self.item_base.wire_count }
            fn fuel_value(&self) -> Energy { self.item_base.fuel_value }
            fn fuel_acceleration_multiplier(&self) -> f64 { self.item_base.fuel_acceleration_multiplier }
            fn fuel_top_speed_multiplier(&self) -> f64 { self.item_base.fuel_top_speed_multiplier }
            fn fuel_emissions_multiplier(&self) -> f64 { self.item_base.fuel_emissions_multiplier }
            fn fuel_glow_color(&self) -> &Color { &self.item_base.fuel_glow_color }
            fn open_sound(&self) -> &Option<Sound> { &self.item_base.open_sound }
            fn close_sound(&self) -> &Option<Sound> { &self.item_base.close_sound }
            fn dark_background_icon(&self) -> &Option<IconSpecification> { &self.item_base.dark_background_icon }
            fn rocket_launch_products(&self) -> &Option<Vec<ItemProductPrototype>> { &self.item_base.rocket_launch_products }
            fn rocket_launch_product(&self) -> &Option<ItemProductPrototype> { &self.item_base.rocket_launch_product }
        }
    };
    gen.into()
}

fn impl_selection_tool_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SelectionTool for #name {
            fn selection_color(&self) -> &Color { &self.selection_tool_base.selection_color }
            fn alt_selection_color(&self) -> &Color { &self.selection_tool_base.alt_selection_color }
            fn selection_mode(&self) -> SelectionMode { self.selection_tool_base.selection_mode }
            fn alt_selection_mode(&self) -> SelectionMode { self.selection_tool_base.alt_selection_mode }
            fn selection_cursor_box_type(&self) -> CursorBoxType { self.selection_tool_base.selection_cursor_box_type }
            fn alt_selection_cursor_box_type(&self) -> CursorBoxType { self.selection_tool_base.alt_selection_cursor_box_type }
            fn reverse_selection_color(&self) -> &Color { &self.selection_tool_base.reverse_selection_color }
            fn selection_count_button_color(&self) -> &Color { &self.selection_tool_base.selection_count_button_color }
            fn alt_selection_count_button_color(&self) -> &Color { &self.selection_tool_base.alt_selection_count_button_color }
            fn reverse_selection_count_button_color(&self) -> &Color { &self.selection_tool_base.reverse_selection_count_button_color }
            fn chart_selection_color(&self) -> &Color { &self.selection_tool_base.chart_selection_color }
            fn chart_alt_selection_color(&self) -> &Color { &self.selection_tool_base.chart_alt_selection_color }
            fn chart_reverse_selection_color(&self) -> &Color { &self.selection_tool_base.chart_reverse_selection_color }
            fn reverse_selection_mode(&self) -> SelectionMode { self.selection_tool_base.reverse_selection_mode }
            fn reverse_selection_cursor_box_type(&self) -> CursorBoxType { self.selection_tool_base.reverse_selection_cursor_box_type }
            fn always_include_tiles(&self) -> bool { self.selection_tool_base.always_include_tiles }
            fn mouse_cursor(&self) -> &String { &self.selection_tool_base.mouse_cursor }
            fn entity_filters(&self) -> &Option<Vec<String>> { &self.selection_tool_base.entity_filters }
            fn alt_entity_filters(&self) -> &Option<Vec<String>> { &self.selection_tool_base.alt_entity_filters }
            fn entity_type_filters(&self) -> &Option<Vec<String>> { &self.selection_tool_base.entity_type_filters }
            fn alt_entity_type_filters(&self) -> &Option<Vec<String>> { &self.selection_tool_base.alt_entity_type_filters }
            fn tile_filters(&self) -> &Option<Vec<String>> { &self.selection_tool_base.tile_filters }
            fn alt_tile_filters(&self) -> &Option<Vec<String>> { &self.selection_tool_base.alt_tile_filters }
            fn entity_filter_mode(&self) -> FilterMode { self.selection_tool_base.entity_filter_mode }
            fn alt_entity_filter_mode(&self) -> FilterMode { self.selection_tool_base.alt_entity_filter_mode }
            fn tile_filter_mode(&self) -> FilterMode { self.selection_tool_base.tile_filter_mode }
            fn alt_tile_filter_mode(&self) -> FilterMode { self.selection_tool_base.alt_tile_filter_mode }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn prot_from_lua_blanket(input: TokenStream) -> TokenStream {
    let target_type = syn::parse::<syn::Type>(input).unwrap();
    let gen = quote! {
        impl<'lua> PrototypeFromLua<'lua> for #target_type {
            fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::prelude::LuaResult<Self> {
                lua.unpack(value)
            }
        }
    };
    gen.into()
}

fn impl_data_table_accessable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attrs = &ast.attrs;

    let mut attrs = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("data_table"))
        .map(|attr| parse_data_table_attribute(attr).expect("failed to parse data_table attribute"));
    let attr = attrs.next().unwrap();
    let gen = quote! {
        impl crate::prototypes::DataTableAccessable for #name {
            fn find<'a>(data_table: &'a crate::prototypes::DataTable, name: &str) -> Result<&'a Self, crate::prototypes::PrototypesErr> {
                data_table.#attr.get(name).ok_or_else(|| crate::prototypes::PrototypesErr::PrototypeNotFound(name.into()))
            }

            fn extend(self, data_table: &mut crate::prototypes::DataTable) -> Result<(), crate::prototypes::PrototypesErr> {
                data_table.#attr.insert(self.name.clone(), self);
                Ok(())
            }
        }
    };
    gen.into()
}

fn parse_data_table_attribute(attr: &Attribute) -> Result<Ident> {
    let field: syn::Path = attr.parse_args()?;
    let ident = field.get_ident().expect("expected indentifier");
    Ok(ident.clone())
}

// TODO: mandatory_if, post-load functions (before returning, but after all fields have been loaded)

// Attribute on field
//
// #[default(expr)] - expr is default value, which is used in case Option<PropertyType> is None
// Incompatible with: use_self, use_self_vec, use_self_forced
//
// #[from_str] - convert value to string, then parse from str
// Incompatible with: resource, use_self, use_self_vec, use_self_forced
//
// #[use_self] - use self-Value for property if corresponding field does not exist
// Incompatible with: default, from_str, use_self_vec, use_self_forced, resource, mandatory_if
//
// #[use_self_vec] - same as use_self, but puts result in a Vec
// Incompatible with: default, from_str, use_self, use_self_forced, resource, mandatory_if
//
// #[use_self_forced] - forced to use self-Value for this field
// Incompatible with: default, from_str, use_self, use_self_vec, resource, mandatory_if
//
// #[resource] - this field is a resource record (sound, textures should be done in post-extraction)
// Incompatible with: from_str, use_self, use_self_vec, use_self_forced
//
// #[mandatory_if(expr)] - expr is a condition, if the condition results in `true`, field value
// must be Some(_)
// Incompatible with: default, use_self, use_self_vec, use_self_forced
//
// #[post_extr_fn(path)] - path is a path to a function that needs to be executed after
// mandatory_if checks
#[proc_macro_derive(PrototypeFromLua, attributes(default, from_str, use_self, use_self_vec, use_self_forced, resource, mandatory_if, post_extr_fn))]
pub fn prototype_from_lua_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_prototype_from_lua_macro(&ast)
}

fn impl_prototype_from_lua_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let str_name = name.to_string();
    let data = { match &ast.data {
        syn::Data::Struct(d) => d,
        _ => panic!("expected struct")
    }};
    let fields = { match &data.fields {
        syn::Fields::Named(f) => f.named.iter(),
        _ => panic!("expected named fields")
    }};
    let (parsed_fields, mut mandatory_exprs): (Vec<proc_macro2::TokenStream>, Vec<Option<proc_macro2::TokenStream>>) = fields
        .clone()
        .map(|f| prot_from_lua_field(f).unwrap())
        .unzip();
    mandatory_exprs.retain(|mex| mex.is_some());
    let mandatory_exprs: Vec<proc_macro2::TokenStream> = mandatory_exprs.into_iter().map(|mex| mex.unwrap()).collect();
    let field_names = fields.map(|f| &f.ident);
    let post_extr_fn: Option<syn::Path> = {
        let mut result = None;
        for attr in &ast.attrs {
            if attr.path.is_ident("post_extr_fn") {
                result = Some(attr.parse_args::<syn::Path>().unwrap())
            }
        }
        result
    };
    let post_extr = if let Some(pef) = post_extr_fn {
        quote! {
            #pef(&result, lua, data_table)?;
        }
    } else {
        quote! { }
    };
    let gen = quote! {
        impl<'lua> crate::prototypes::PrototypeFromLua<'lua> for #name {
            fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::prelude::LuaResult<Self> {
                let str_name = #str_name;
                if let mlua::Value::Table(ref prot_table) = value {
                    #(#parsed_fields)*
                    #(#mandatory_exprs)*
                    let result = Self{#(#field_names),*};
                    #post_extr
                    Ok(result)
                } else {
                    Err(mlua::Error::FromLuaConversionError{from: value.type_name(), to: str_name,
                    message: Some("Expected Table".into())})
                }
            }
        }
    };
    gen.into()
}

struct PrototypeFromLuaFieldAttrArgs {
    default_value: Option<proc_macro2::TokenStream>, // Incompatible with: use_self, use_self_vec
    mandatory_if: Option<proc_macro2::TokenStream>,  // Incompatible with: default, use_self, use_self_vec
    // use_self* is incompatible with default and mandatory_if
    // Only 1 can be used:
    use_from_str: bool,
    use_self: bool,
    use_self_vec: bool,
    use_self_forced: bool,
    is_resource: bool
}

impl PrototypeFromLuaFieldAttrArgs {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        for attr in attrs {
            if attr.path.is_ident("default") {
                if result.use_self { return Self::attr_error(attr, "`default()` is incompatible with `use_self`") }
                if result.use_self_vec { return Self::attr_error(attr, "`default()` is incompatible with `use_self_vec`") }
                if result.use_self_forced { return Self::attr_error(attr, "`default()` is incompatible with `use_self_forced`") }
                if result.mandatory_if.is_some() { return Self::attr_error(attr, "`default()` is incompatible with `mandatory_if()`") }
                result.default_value = Some(attr.tokens.clone())
            } else if attr.path.is_ident("mandatory_if") {
                if result.default_value.is_some() { return Self::attr_error(attr, "`mandatory_if()` attribute is incompatible with `default()`") }
                if result.use_self { return Self::attr_error(attr, "`mandatory_if()` is incompatible with `use_self`") }
                if result.use_self_vec { return Self::attr_error(attr, "`mandatory_if()` is incompatible with `use_self_vec`") }
                if result.use_self_forced { return Self::attr_error(attr, "`mandatory_if()` is incompatible with `use_self_forced`") }
                result.mandatory_if = Some(attr.tokens.clone())
            } else if attr.path.is_ident("from_str") {
                if result.is_resource { return Self::attr_error(attr, "`from_str` attribute is incompatible with `resource`") }
                if result.use_self { return Self::attr_error(attr, "`from_str` is incompatible with `use_self`") }
                if result.use_self_vec { return Self::attr_error(attr, "`from_str` is incompatible with `use_self_vec`") }
                if result.use_self_forced { return Self::attr_error(attr, "`from_str` is incompatible with `use_self_forced`") }
                result.use_from_str = true
            } else if attr.path.is_ident("use_self") {
                if result.default_value.is_some() { return Self::attr_error(attr, "`use_self` is incompatible with `default()`") }
                if result.use_from_str { return Self::attr_error(attr, "`use_self` is incompatible with `from_str`") }
                if result.use_self_vec { return Self::attr_error(attr, "`use_self` is incompatible with `use_self_vec`") }
                if result.use_self_forced { return Self::attr_error(attr, "`use_self` is incompatible with `use_self_forced`") }
                if result.mandatory_if.is_some() { return Self::attr_error(attr, "`use_self` is incompatible with `mandatory_if()`") }
                result.use_self = true
            } else if attr.path.is_ident("use_self_vec") {
                if result.default_value.is_some() { return Self::attr_error(attr, "`use_self_vec` is incompatible with `default()`") }
                if result.use_from_str { return Self::attr_error(attr, "`use_self_vec` is incompatible with `from_str`") }
                if result.use_self { return Self::attr_error(attr, "`use_self_vec` is incompatible with `use_self`") }
                if result.use_self_forced { return Self::attr_error(attr, "`use_self_vec` is incompatible with `use_self_forced`") }
                if result.mandatory_if.is_some() { return Self::attr_error(attr, "`use_self_vec` is incompatible with `mandatory_if()`") }
                result.use_self = true
            } else if attr.path.is_ident("use_self_forced") {
                if result.default_value.is_some() { return Self::attr_error(attr, "`use_self_forced` is incompatible with `default()`") }
                if result.use_from_str { return Self::attr_error(attr, "`use_self_forced` is incompatible with `from_str`") }
                if result.use_self { return Self::attr_error(attr, "`use_self_forced` is incompatible with `use_self`") }
                if result.use_self_vec { return Self::attr_error(attr, "`use_self_forced` is incompatible with `use_self_vec`") }
                if result.mandatory_if.is_some() { return Self::attr_error(attr, "`use_self_forced` is incompatible with `mandatory_if()`") }
                result.use_self_forced = true
            } else if attr.path.is_ident("resource") {
                if result.use_from_str { return Self::attr_error(attr, "`resource` is incompatible with `from_str`") }
                if result.use_self { return Self::attr_error(attr, "`resource` is incompatible with `use_self_vec`") }
                if result.use_self_vec { return Self::attr_error(attr, "`resource` is incompatible with `use_self_vec`") }
                if result.use_self_forced { return Self::attr_error(attr, "`resource` is incompatible with `use_self_forced`") }
                result.is_resource = true
            }
        }
        Ok(result)
    }

    fn attr_error<T: Display>(attr: &Attribute, message: T) -> Result<Self> {
        Err(syn::Error::new(attr.span(), message))
    }
}

impl Default for PrototypeFromLuaFieldAttrArgs {
    fn default() -> Self {
        Self{
            default_value: None, 
            mandatory_if: None,
            use_from_str: false,
            use_self: false,
            use_self_vec: false,
            use_self_forced: false,
            is_resource: false
        }
    }
}

fn prot_from_lua_field(field: &syn::Field) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> { // First is get_expr, second is mandatory_if
    let ident = &field.ident;
    let str_field = ident.as_ref().unwrap().to_string();
    let field_type = &field.ty;
    let prototype_field_attrs = PrototypeFromLuaFieldAttrArgs::from_attrs(&field.attrs)?;
    let mut field_extr_type = if prototype_field_attrs.use_from_str || prototype_field_attrs.is_resource {
        quote! { String }
    } else {
        quote! { #field_type }
    };
    if prototype_field_attrs.default_value.is_some() {
        field_extr_type = quote! { Option<#field_extr_type> };
    }
    let field_get_expr = if let Some(def_val) = prototype_field_attrs.default_value {
        quote! { prot_table.get_prot::<_, #field_extr_type>(#str_field, lua, data_table)?.unwrap_or_else(|| #def_val.into()) }
    } else {
        quote! { prot_table.get_prot::<_, #field_extr_type>(#str_field, lua, data_table)? }
    };
    let get_self = quote! {
        crate::prototypes::PrototypeFromLua::prototype_from_lua(value.clone(), lua, data_table)
    };
    let get_expr = if prototype_field_attrs.use_self_forced {
        quote! {
            #get_self?;
        }
    } else if prototype_field_attrs.is_resource {
        quote! {
            {
                let name = #field_get_expr;
                data_table.register_resource(crate::prototypes::ResourceRecord{path: name.clone(), resource_type: crate::prototypes::ResourceType::Sound});
                name.into()
            };
        }
    } else if prototype_field_attrs.use_from_str {
        quote! { #field_get_expr.parse::<#field_type>().map_err(mlua::Error::external)?; }
    } else if prototype_field_attrs.use_self_vec {
        quote! { 
            prot_table.get_prot::<_, Option<#field_extr_type>>(#str_field, lua, data_table).transpose()
                .unwrap_or_else(|| Ok(Vec::from([#get_self?])))?;
        }
    } else if prototype_field_attrs.use_self {
        quote! {
            prot_table.get_prot::<_, Option<#field_extr_type>>(#str_field, lua, data_table).transpose()
                .unwrap_or_else(|| #get_self)?;
        }
    } else {
        quote! { #field_get_expr; }
    };
    let mand_expr = if let Some(mandatory_if) = prototype_field_attrs.mandatory_if {
        let err_str = format!("{} is required", ident.clone().unwrap());
        Some(quote! {
            if #mandatory_if && #ident.is_none() {
                return Err(mlua::Error::FromLuaConversionError{from: value.type_name(), to: str_name, message: Some(#err_str.into())})
            };
        })
    } else {
        None
    };
    let gen = quote! {
        let #ident: #field_type = #get_expr
    };
    Ok((gen, mand_expr))
}
