use mlua::ToLua;
use strum::IntoEnumIterator;
use std::convert::AsRef;
use strum::{EnumDiscriminants, EnumString, AsRefStr, EnumIter};

use crate::prototypes::GetPrototype;
use crate::prototypes::PrototypeFromLua;

pub type NoiseExpression = String;

/// <https://wiki.factorio.com/Types/AutoplaceSpecification>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct AutoplaceSpecification {
    #[default("")]
    pub control: String, // Default: "" // id of autoplace control
    #[default(true)]
    pub default_enabled: bool, // Default: true
    #[default("neutral")]
    pub force: String, // Default: "neutral"
    #[default("")]
    pub order: String, // Default: ""
    #[default(1_u32)]
    pub placement_density: u32, // Default: 1
    #[default(vec![])]
    pub tile_restriction: Vec<TileRestriction>, // Default: empty // Official docs are not clear about what this actually is, assuming it's a list of String
    #[use_self_forced]
    pub base: AutoplaceSpecificationBase,
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#tile_restriction>
#[derive(Debug, Clone)]
pub enum TileRestriction {
    Single(String),
    OnTransitions([String; 2])
}

impl<'lua> PrototypeFromLua<'lua> for TileRestriction {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::Result<Self> {
        if let Some(s) = lua.unpack::<Option<String>>(value.clone())? {
            Ok(Self::Single(s))
        } else if let Some(v) = lua.unpack::<Option<[String; 2]>>(value)? {
            Ok(Self::OnTransitions(v))
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "AutoplaceSpecification.tile_restriction", message: Some("Expected eitehr a string or an array of two strings".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#General_properties>
#[derive(Debug, Clone)]
pub enum AutoplaceSpecificationBase {
    /// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
    Expression(ExpressionBasedAutoplaceSpecification),
    /// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
    Peak(PeakBasedAutoplaceSpecification)
}

impl<'lua> PrototypeFromLua<'lua> for AutoplaceSpecificationBase {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(t) = &value {
            if t.contains_key("probability_expression")? {
                Ok(Self::Expression(ExpressionBasedAutoplaceSpecification::prototype_from_lua(value, lua, data_table)?))
            } else {
                Ok(Self::Peak(PeakBasedAutoplaceSpecification::prototype_from_lua(value, lua, data_table)?))
            }
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "AutoplaceSpecification", message: Some("Expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ExpressionBasedAutoplaceSpecification {
    probability_expression: NoiseExpression,
    #[default(probability_expression.clone())]
    richness_expression: NoiseExpression
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct PeakBasedAutoplaceSpecification {
    #[default(0_f64)]
    sharpness: f64, // Default: 0
    #[default(1_f64)]
    max_probability: f64, // Default: 1
    #[default(0_f64)]
    richness_base: f64, // Default: 0
    #[default(0_f64)]
    richness_multiplier: f64, // Default: 0
    #[default(0_f64)]
    richness_multiplier_distance_bonus: f64, // Default: 0
    #[default(0_f64)]
    random_probability_penalty: f64, // Default: 0
    #[use_self_vec]
    peaks: Vec<AutoplacePeak>, // If not specified, interpret specification as peak
    #[default(0_f64)] // FIXME
    coverage: f64, // Default: calculated from existing peaks // What
    #[default(0_u32)]
    starting_area_amount: u32, // Default: 0
    #[default(10_f64)]
    starting_area_size: f64, // Default: 10
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Autoplace_peaks>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct AutoplacePeak {
    #[default(1.0_f64)]
    influence: f64, // Default: 1
    #[default(f64::MIN)]
    min_influence: f64, // Default: f64::MIN
    #[default(f64::MAX)]
    max_influence: f64, // Default: f64::MAX
    #[default(0)]
    richness_influence: f64, // Default: 0
    #[default("")]
    noise_layer: String, // Default: ""
    #[default(0.5_f64)]
    noise_persistence: f64, // Default: 0.5
    #[default(0)]
    noise_octaves_difference: f64, // Default: 0
    #[default(1)]
    noise_scale: f64, // Default: 1
    #[use_self_forced]
    dimensions: Dimensions // Default: empty // Only one of each type
}

#[derive(Debug, Clone)]
pub struct Dimensions(Vec<Dimension>);

impl<'lua> PrototypeFromLua<'lua> for Dimensions {
    fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = &value {
            let mut result = Vec::new();
            for dimension in DimensionDiscriminants::iter() {
                let mut dim_table = lua.create_table()?;
                if let Some(optimal) = table.get::<_, Option<f64>>(format!("{}_optimal", dimension.as_ref()))? {
                    dim_table.set("optimal", optimal)?; }
                if let Some(range) = table.get::<_, Option<f64>>(format!("{}_range", dimension.as_ref()))? {
                    dim_table.set("range", range)?; }
                if let Some(max_range) = table.get::<_, Option<f64>>(format!("{}_max_range", dimension.as_ref()))? {
                    dim_table.set("max_range", max_range)?; }
                if let Some(top_property_limit) = table.get::<_, Option<f64>>(format!("{}_top_property_limit", dimension.as_ref()))? {
                    dim_table.set("top_property_limit", top_property_limit)?; }
                if let Ok(dimension_val) = DimensionSpec::prototype_from_lua(dim_table.to_lua(lua)?, lua, data_table) {
                    result.push(match dimension {
                        DimensionDiscriminants::StartingAreaWeight => Dimension::StartingAreaWeight(dimension_val),
                        DimensionDiscriminants::Elevation => Dimension::Elevation(dimension_val),
                        DimensionDiscriminants::Water => Dimension::Water(dimension_val),
                        DimensionDiscriminants::Temperature => Dimension::Temperature(dimension_val),
                        DimensionDiscriminants::Aux => Dimension::Aux(dimension_val),
                        DimensionDiscriminants::TierFromStart => Dimension::TierFromStart(dimension_val),
                        DimensionDiscriminants::Distance => Dimension::Distance(dimension_val),
                    })
                }
            }
            Ok(Self(result))
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Dimensions",
            message: Some("expected table. You shouldn't be able to get this error".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Dimensions>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, AsRefStr, EnumIter), strum(serialize_all="snake_case"))]
pub enum Dimension {
    StartingAreaWeight(DimensionSpec),
    Elevation(DimensionSpec),
    Water(DimensionSpec),
    Temperature(DimensionSpec),
    Aux(DimensionSpec),
    TierFromStart(DimensionSpec),
    Distance(DimensionSpec),
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Dimensions>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct DimensionSpec {
    optimal: Option<f64>,
    #[default(0)]
    range: f64, // Default: 0
    // ~~Default: range * 1.5 // Default value taken from Factorio base mod source code, version 1.1.37, decoratives.lua, lines 11-17~~
    // It's in code, you idiot, not in loader!
    max_range: Option<f64>,
    #[default(f64::MAX)]
    top_property_limit: f64, // Default: f64::MAX // Seems to be unused (in vanilla)
}

/* TODO
/// <https://wiki.factorio.com/Types/NoiseExpression>
#[derive(Debug, Clone)]
pub enum NoiseExpression {
    Variable(String), // variable_name
    FunctionApplication(String, String), // function_name and arguments //  FIXME // This does not actually satisfy the api, because arguments make my brain explode
    LiteralBoolean(bool), // literal_value
    LiteralNumber(f32), // literal_value
    LiteralString(String), // literal_value
    LiteralObject(String), // FIXME // I'm not going to implement this properly.
    LiteralExpression(Box<NoiseExpression>), // literal_value // oh god no
    ArrayConstruction(Vec<NoiseExpression>), // FIXME // Not implemented properly
    ProcedureDelimeter(Box<NoiseExpression>), // expression
    IfElseChain(String), // FIXME // no
}
*/
