extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

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
    impl_entity_macro(&ast)
}

#[proc_macro_derive(Corpse)]
pub fn corpse_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_corpse_macro(&ast)
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
