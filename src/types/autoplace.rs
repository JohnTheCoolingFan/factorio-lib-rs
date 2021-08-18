/// <https://wiki.factorio.com/Types/AutoplaceSpecification>
#[derive(Debug)]
pub struct AutoplaceSpecification {
    control: String, // Default: "" // id of autoplace control
    default_enabled: bool, // Default: true
    force: String, // Default: "neutral"
    order: String, // Default: ""
    placement_density: u32, // Default: 1
    tile_restriction: Vec<String>, // Default: empty // Official docs are not clear about what this actually is, assuming it's a list of String
    base: AutoplaceSpecificationBase,
}

#[derive(Debug)]
pub enum AutoplaceSpecificationBase {
    /// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
    Expression(ExpressionBasedAutoplaceSpecification),
    /// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
    Peak(PeakBasedAutoplaceSpecification)
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
#[derive(Debug)]
pub struct ExpressionBasedAutoplaceSpecification {
    probability_expression: NoiseExpression,
    richness_expression: NoiseExpression
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Properties_for_Peak-based_AutoplaceSpecifications>
#[derive(Debug)]
pub struct PeakBasedAutoplaceSpecification {
    sharpness: f64, // Default: 0
    max_probability: f64, // Default: 1
    richness_base: f64, // Default: 0
    richness_multiplier: f64, // Default: 0
    richness_multiplier_distance_bonus: f64, // Default: 0
    random_probability_penalty: f64, // Default: 0
    peaks: Vec<AutoplacePeak>, // If not specified, interpret specification as peak
    coverage: f64, // Default: calculated from existing peaks // What
    starting_area_amount: u32, // Default: 0
    starting_area_size: f64, // Default: 10
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Autoplace_peaks>
#[derive(Debug)]
pub struct AutoplacePeak {
    influence: f64, // Default: 1
    min_influence: f64, // Default: f64::MIN
    max_influence: f64, // Default: f64::MAX
    richness_influence: f64, // Default: 0
    noise_layer: String, // Default: ""
    noise_persistence: f64, // Default: 0.5
    noise_octaves_difference: f64, // Default: 0
    noise_scale: f64, // Default: 1
    dimensions: Vec<Dimension> // Default: empty // Only one of each type
}

/// <https://wiki.factorio.com/Types/AutoplaceSpecification#Dimensions>
#[derive(Debug)]
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
#[derive(Debug)]
pub struct DimensionSpec {
    optimal: f64,
    range: f64, // Default: 0
    max_range: f64, // Default: range * 1.5 // Default value taken from Factorio base mod source code, version 1.1.37, decoratives.lua, lines 11-17
    top_property_limit: f64, // Default: f64::MAX // Seems to be unused
}

/// <https://wiki.factorio.com/Types/NoiseExpression>
#[derive(Debug)]
pub enum NoiseExpression {
    Variable(String), // variable_name
    FunctionApplication(String, String), // function_name and arguments //  FIXME // This does not actually satisfy the api, because arguments make my brain explode
    LiteralBoolean(bool), // literal_value
    LiteralNumber(f32), // literal_value
    LiteralString(String), // literal_value
    LiteralObject(String), // FIXME // I'm not going to implement this properly.
    LiteralExpression(Box<NoiseExpression>), // literal_value // oh god no
    ArrayConstruction(Vec<Box<NoiseExpression>>), // FIXME // Not implemented properly
    ProcedureDelimeter(Box<NoiseExpression>), // expression
    IfElseChain(String), // FIXME // no
}
