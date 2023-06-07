use crate::util::defaults::*;
use mlua::ToLua;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::AsRef;
use strum::IntoEnumIterator;
use strum::{AsRefStr, EnumDiscriminants, EnumIter, EnumString};

use crate::prototypes::{GetPrototype, PrototypeFromLua};

pub type NoiseExpression = String;

/// <https://wiki.factorio.com/Types/AutoplaceSpecification>
#[derive(Debug, Clone, Deserialize)]
pub struct AutoplaceSpecification {
    #[serde(default)]
    pub control: String, // Default: "" // id of autoplace control
    #[serde(default = "default_bool::<true>")]
    pub default_enabled: bool, // Default: true
    #[serde(default = "default_string_neutral")]
    pub force: String, // Default: "neutral"
    #[serde(default)]
    pub order: String, // Default: ""
    #[serde(default = "default_u32::<1>")]
    pub placement_density: u32, // Default: 1
    #[serde(default)]
    pub tile_restriction: Vec<TileRestriction>, // Default: empty
    #[serde(flatten)]
    pub base: AutoplaceSpecificationBase,
}

fn default_string_neutral() -> String {
    "neutral".into()
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#tile_restriction>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TileRestriction {
    Single(String),
    OnTransitions([String; 2]),
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#General_properties>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AutoplaceSpecificationBase {
    /// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
    Expression(ExpressionBasedAutoplaceSpecification),
    /// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
    Peak(PeakBasedAutoplaceSpecification),
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Expression-based_AutoplaceSpecifications>
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "ExpressionBasedAutoplaceSpecificationIntermediate")]
pub struct ExpressionBasedAutoplaceSpecification {
    pub probability_expression: NoiseExpression,
    pub richness_expression: NoiseExpression,
}

#[derive(Deserialize)]
struct ExpressionBasedAutoplaceSpecificationIntermediate {
    pub probability_expression: NoiseExpression,
    pub richness_expression: Option<NoiseExpression>,
}

impl From<ExpressionBasedAutoplaceSpecificationIntermediate>
    for ExpressionBasedAutoplaceSpecification
{
    fn from(value: ExpressionBasedAutoplaceSpecificationIntermediate) -> Self {
        Self {
            probability_expression: value.probability_expression,
            richness_expression: value
                .richness_expression
                .unwrap_or_else(|| value.probability_expression.clone()),
        }
    }
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
#[derive(Debug, Clone, Deserialize)]
pub struct PeakBasedAutoplaceSpecification {
    #[serde(default)]
    pub sharpness: f64, // Default: 0
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub max_probability: f64, // Default: 1
    #[serde(default)]
    pub richness_base: f64, // Default: 0
    #[serde(default)]
    pub richness_multiplier: f64, // Default: 0
    #[serde(default)]
    pub richness_multiplier_distance_bonus: f64, // Default: 0
    #[serde(default)]
    pub random_probability_penalty: f64, // Default: 0
    #[serde(flatten)]
    pub peaks: AutoplacePeaks, // If not specified, interpret specification as peak
    #[serde(default)] // FIXME
    pub coverage: f64, // Default: calculated from existing peaks // What
    #[serde(default)]
    pub starting_area_amount: u32, // Default: 0
    #[serde(default = "default_from_i8::<f64, 10>")]
    pub starting_area_size: f64, // Default: 10
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AutoplacePeaks {
    Single(AutoplacePeak),
    Multiple { peaks: Vec<AutoplacePeak> },
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Autoplace_peaks>
#[derive(Debug, Clone, Deserialize)]
pub struct AutoplacePeak {
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub influence: f64, // Default: 1
    #[serde(default = "default_f64_min")]
    pub min_influence: f64, // Default: f64::MIN
    #[serde(default = "default_f64_max")]
    pub max_influence: f64, // Default: f64::MAX
    #[serde(default)]
    pub richness_influence: f64, // Default: 0
    #[serde(default)]
    pub noise_layer: String, // Default: ""
    #[serde(default = "default_f64_0_5")]
    pub noise_persistence: f64, // Default: 0.5
    #[serde(default)]
    pub noise_octaves_difference: f64, // Default: 0
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub noise_scale: f64, // Default: 1
    #[serde(flatten)]
    // TODO pub dimensions: Dimensions, // Default: empty // Only one of each type
    pub dimensions: HashMap<String, f64>,
}

const fn default_f64_min() -> f64 {
    f64::MIN
}

const fn default_f64_max() -> f64 {
    f64::MAX
}

const fn default_f64_0_5() -> f64 {
    0.5
}

// TODO: https://wiki.factorio.com/Types/AutoplaceSpecification#Dimensions
#[derive(Debug, Clone)]
pub struct Dimensions(Vec<Dimension>);

impl<'lua> PrototypeFromLua<'lua> for Dimensions {
    fn prototype_from_lua(
        value: mlua::Value<'lua>,
        lua: &'lua mlua::Lua,
        data_table: &mut crate::prototypes::DataTable,
    ) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = &value {
            let mut result = Vec::new();
            for dimension in DimensionDiscriminants::iter() {
                let dim_table = lua.create_table()?;
                if let Some(optimal) =
                    table.get::<_, Option<f64>>(format!("{}_optimal", dimension.as_ref()))?
                {
                    dim_table.set("optimal", optimal)?;
                }
                if let Some(range) =
                    table.get::<_, Option<f64>>(format!("{}_range", dimension.as_ref()))?
                {
                    dim_table.set("range", range)?;
                }
                if let Some(max_range) =
                    table.get::<_, Option<f64>>(format!("{}_max_range", dimension.as_ref()))?
                {
                    dim_table.set("max_range", max_range)?;
                }
                if let Some(top_property_limit) = table
                    .get::<_, Option<f64>>(format!("{}_top_property_limit", dimension.as_ref()))?
                {
                    dim_table.set("top_property_limit", top_property_limit)?;
                }
                if let Ok(dimension_val) =
                    DimensionSpec::prototype_from_lua(dim_table.to_lua(lua)?, lua, data_table)
                {
                    result.push(match dimension {
                        DimensionDiscriminants::StartingAreaWeight => {
                            Dimension::StartingAreaWeight(dimension_val)
                        }
                        DimensionDiscriminants::Elevation => Dimension::Elevation(dimension_val),
                        DimensionDiscriminants::Water => Dimension::Water(dimension_val),
                        DimensionDiscriminants::Temperature => {
                            Dimension::Temperature(dimension_val)
                        }
                        DimensionDiscriminants::Aux => Dimension::Aux(dimension_val),
                        DimensionDiscriminants::TierFromStart => {
                            Dimension::TierFromStart(dimension_val)
                        }
                        DimensionDiscriminants::Distance => Dimension::Distance(dimension_val),
                    })
                }
            }
            Ok(Self(result))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Dimensions",
                message: Some("expected table. You shouldn't be able to get this error".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Dimensions>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(
    derive(EnumString, AsRefStr, EnumIter),
    strum(serialize_all = "snake_case")
)]
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
    pub optimal: Option<f64>,
    #[default(0)]
    pub range: f64, // Default: 0
    // ~~Default: range * 1.5 // Default value taken from Factorio base mod source code, version 1.1.37, decoratives.lua, lines 11-17~~
    // It's in code, you idiot, not in loader!
    pub max_range: Option<f64>,
    #[default(f64::MAX)]
    pub top_property_limit: f64, // Default: f64::MAX // Seems to be unused (in vanilla)
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
