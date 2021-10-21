use crate::concepts::LocalisedString;
use crate::types::{Color, Sprite, FileName, Position, SpriteSizeType, Sound};
use strum_macros::{EnumString, AsRefStr};

// FIXME?
// There is a big problem for recreating prototype API from public documentation which is intended
// for documenting how to use API, not reverse-engineer it. Many optional properties don't have
// default values, and it is not known (until further manual testing or documentation update)
// which of them are optional from API stand point and what their default values are.
// For now, I'll leave some properties that I think do have a default value or are mandatory
// without Option<> wrap.

/// https://wiki.factorio.com/Types/StyleSpecification
#[derive(Debug)]
pub enum StyleSpecification {
    ActivityBar(ActivityBarStyleSpecification),
    Camera(CameraStyleSpecification),
    Minimap(MinimapStyleSpecification),
    DropDown(DropDownStyleSpecification),
    Flow(FlowStyleSpecification),
    Frame(FrameStyleSpecification),
    Glow(GlowStyleSpecification),
    Graph(GraphStyleSpecification),
    HorizontalFlow(HorizontalFlowStyleSpecification),
    Image(ImageStyleSpecification),
    Label(LabelStyleSpecification),
    Line(LineStyleSpecification),
    ListBox(ListBoxStyleSpecification),
    ProgressBar(ProgressBarStyleSpecification),
    HorizontalScrollBar(HorizontalScrollBarStyleSpecification),
    VerticalScrollBar(VerticalScrollBarStyleSpecification),
    ScrollPane(ScrollPaneStyleSpecification),
    Slider(SliderStyleSpecification),
    DoubleSlider(DoubleSliderStyleSpecification),
    SpeechBubble(SpeechBubbleStyleSpecification),
    Button(ButtonStyleSpecification),
    TechnologySlot(TechnologySlotStyleSpecification),
    CheckBox(CheckBoxStyleSpecification),
    RadioButton(RadioButtonStyleSpecification),
    Switch(SwitcjStyleSpecification),
    Tabbed(TabbedStyleSpecification),
    Table(TableStyleSpecification),
    Tab(TabStyleSpecification),
    TextBox(TextBoxStyleSpecification),
    VerticalFlow(VerticalFlowStyleSpecification)
}

/// <https://wiki.factorio.com/Types/StyleSpecification>
#[derive(Debug)]
pub struct StyleSpecificationBase {
    parent: String, // Name of StyleSpecification
    horizontal_align: HorizontalAlignment, // Default: "left"
    vertical_align: VerticalAlignment, // Default: "top"
    ignored_by_search: Option<bool>,
    never_hide_by_search: Option<bool>,
    horizontally_stretchable: StretchRule, // Default: "auto"
    vertically_stretchable: StretchRule, // Default: "auto"
    horizontally_squashable: StretchRule, // Default: "auto"
    vertically_squashable: StretchRule, // Default: "auto"
    natural_size: [u32; 2], // combines `natural_width` and `natural_height`
    size: [u32; 2], // combines `width` and `height` // Default 0
    minimal_size: [u32; 2], // Same here
    padding: [i16; 4], // top, right, bottom and left // Default 0
    margin: [i16; 4], // top, eight, bottom and left // Default 0
    effect: String,
    effect_opacity: f32, // Default: 1
    tooltip: LocalisedString
}

/// <https://wiki.factorio.com/Types/StyleSpecification#horizontal_align>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

/// <https://wiki.factorio.com/Types/StyleSpecification#vertical_align>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

/// <https://wiki.factorio.com/Types/StretchRule>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum StretchRule {
    On,
    Off,
    Auto,
    StretchAndExpand,
}

/// <https://wiki.factorio.com/Types/ActivityBarStyleSpecification>
#[derive(Debug)]
pub struct ActivityBarStyleSpecification {
    base: StyleSpecificationBase,
    speed: f32,
    bar_width: u32,
    color: Color,
    bar_background: Sprite,
    bar: Sprite,
    bar_size_ratio: f32
}

/// <https://wiki.factorio.com/Types/EmptyWidgetStyleSpecification>
#[derive(Debug)]
pub enum EmptyWidgetStyleSpecification {
    Camera(CameraStyleSpecification),
    Minimap(MinimapStyleSpecification)
}

/// <https://wiki.factorio.com/Types/EmptyWidgetStyleSpecification>
/// <https://wiki.factorio.com/Types/CameraStyleSpecification>
#[derive(Debug)]
pub struct CameraStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: ElementImageSet
}

/// <https://wiki.factorio.com/Types/EmptyWidgetStyleSpecification>
/// <https://wiki.factorio.com/Types/MinimapStyleSpecification>
#[derive(Debug)]
pub struct MinimapStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: ElementImageSet
}

/// <https://wiki.factorio.com/Types/ElementImageSet>
#[derive(Debug)]
pub struct ElementImageSet {
    base: ElementImageSetLayer,
    shadow: Option<ElementImageSetLayer>,
    glow: Option<ElementImageSetLayer>
}

/// <https://wiki.factorio.com/Types/ElementImageSetLayer>
#[derive(Debug)]
pub struct ElementImageSetLayer {
    draw_type: DrawType, // Default: "inner"
    eisl_type: ElementImageSetLayerType, // Default: "none" if there are no other properties, "composition" otherwise
    // Only loaded if `type` is "composition" section start
    tint: Color, // Default: all 1
    center: Option<Sprite>, // if no other section properties set (incuding this one), entire ElementImageSetLayer is loaded as Sprite and gets used as `center`
    left: Option<Sprite>,
    left_top: Option<Sprite>,
    left_bottom: Option<Sprite>,
    right: Option<Sprite>,
    right_top: Option<Sprite>,
    right_bottom: Option<Sprite>,
    top: Option<Sprite>,
    bottom: Option<Sprite>,
    corner_size: Option<(u16, u16)>, // If this is a number, load as both
    // Only loaded if `corner_size` is not None section start
    filename: Option<FileName>, // Default: `default_tileset` in Prototype/GuiStyle
    position: Option<Position>, // Mandatory if `corner_size exists`
    load_in_minimal_mode: bool, // Default: true
    top_width: SpriteSizeType, // Default: 1
    bottom_width: SpriteSizeType, // Default: 1
    left_height: SpriteSizeType, // Deault: 1
    right_height: SpriteSizeType, // Default: 1
    center_width: SpriteSizeType, // Default: 1
    center_height: SpriteSizeType, // Default: 1
    scale: f64, // Default: 1
    // Only loaded if `corner_size` is not None section end
    border: Option<[i32; 4]>, // `top_border`, `right_border`, `bottom_border`, `left_border` // Default 0 probably
    stretch_monolith_image_to_size: bool, // Default: true
    // Only loaded if `type` is "composition" section end
    left_tiling: bool, // Default: false
    right_tiling: bool, // Default: false
    top_tiling: bool, // Default: false
    bottom_tiling: bool, // Default: false
    center_tiling_vertical: bool, // Default: false
    center_tiling_horizontal: bool, // Default: false
    overall_tiling_horizontal_size: u16, // Default: 0
    overall_tiling_horizontal_spacing: u16, // Default: 0
    overall_tiling_horizontal_padding: u16, // Default: 0
    overall_tiling_vertical_size: u16, // Default: 0
    overall_tiling_vertical_spacing: u16, // Default: 0
    overall_tiling_vertical_padding: u16, // Default: 0
    custom_horizontal_tiling_sizes: Option<Vec<u32>>,
    opacity: f64, // Default: 1
    background_blur: bool, // Default: false
    background_blur_sigma: f32, // Default: 4 if `background_blur` is true, doesn't matter otherwise but I'll set to 0
    top_outer_border_shift: i32, // Default: 0
    bottom_outer_border_shift: i32, // Default: 0
    right_outer_border_shift: i32, // Default: 0
    left_outer_border_shift: i32 // Default: 0
}

/// <https://wiki.factorio.com/Types/ElementImageSetLayer#draw_type>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DrawType {
    Inner,
    Outer,
}

/// <https://wiki.factorio.com/Types/ElementImageSetLayer#type>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ElementImageSetLayerType {
    None,
    Composition,
}

/// <https://wiki.factorio.com/Types/DropDownStyleSpecification>
#[derive(Debug)]
pub struct DropDownStyleSpecification {
    base: StyleSpecificationBase,
    button_style: Option<ButtonStyleSpecification>,
    icon: Option<Sprite>,
    list_box_style: Option<ListBoxStyleSpecification>,
    selector_and_title_spacing: i16, // Default: 0 (unconfirmed)
    opened_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Types/FlowStyleSpecification>
#[derive(Debug)]
pub struct FlowStyleSpecification {
    base: StyleSpecificationBase,
    // Default 0 (unconfirmed)
    max_on_row: i32,
    horizontal_spacing: i32,
    vertical_spacing: i32
}

/// <https://wiki.factorio.com/Types/FrameStyleSpecification>
#[derive(Debug)]
pub struct FrameStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: Option<ElementImageSet>,
    flow_style: Option<FlowStyleSpecification>,
    horizontal_flow_style: Option<HorizontalFlowStyleSpecification>,
    vertical_flow_style: Option<VerticalFlowStyleSpecification>,
    header_flow_style: Option<HorizontalFlowStyleSpecification>,
    header_filler_style: Option<EmptyWidgetStyleSpecification>,
    title_style: Option<LabelStyleSpecification>,
    use_header_filler: bool,
    drag_by_title: bool,
    header_background: Option<ElementImageSet>,
    background_graphical_set: Option<ElementImageSet>,
    border: Option<BorderImageSet>
}

/// <https://wiki.factorio.com/Types/BorderImageSet>
#[derive(Debug)]
pub struct BorderImageSet {
    scale: f64, // Default: 1
    border_width: u32, // Default: 0
    vertical_line: Option<Sprite>,
    horizontal_line: Option<Sprite>,
    top_right_corner: Option<Sprite>,
    bottom_right_corner: Option<Sprite>,
    bottom_left_corner: Option<Sprite>,
    top_left_coner: Option<Sprite>,
    top_t: Option<Sprite>,
    right_t: Option<Sprite>,
    bottom_t: Option<Sprite>,
    left_t: Option<Sprite>,
    cross: Option<Sprite>,
    top_end: Option<Sprite>,
    right_end: Option<Sprite>,
    bottom_end: Option<Sprite>,
    left_end: Option<Sprite>
}

/// <https://wiki.factorio.com/Types/GlowStyleSpecification>
#[derive(Debug)]
pub struct GlowStyleSpecification {
    base: StyleSpecificationBase,
    image_set: Option<ElementImageSet>
}

/// <https://wiki.factorio.com/Types/GraphStyleSpecification>
#[derive(Debug)]
pub struct GraphStyleSpecification {
    base: StyleSpecificationBase,
    background_color: Color,
    line_colors: Vec<Color>,
    horizontal_label_style: Option<LabelStyleSpecification>,
    vertical_label_style: Option<LabelStyleSpecification>,
    minimal_horizontal_label_spacing: u32,
    minimal_vertical_label_spacing: u32,
    horizontal_labels_margin: u32,
    vertical_labels_margin: u32,
    graph_top_margin: u32,
    graph_right_margin: u32,
    data_line_highlight_distance: u32,
    selection_dot_radius: u32,
    grid_lines_color: Color,
    guide_lines_color: Color
}

/// <https://wiki.factorio.com/Types/HorizontalFlowStyleSpecification>
#[derive(Debug)]
pub struct HorizontalFlowStyleSpecification {
    base: StyleSpecificationBase,
    horizontal_spacing: i32
}

/// <https://wiki.factorio.com/Types/VerticalFlowStyleSpecification>
#[derive(Debug)]
pub struct VerticalFlowStyleSpecification {
    base: StyleSpecificationBase,
    vertical_spacing: i32
}

/// <https://wiki.factorio.com/Types/ImageStyleSpecification>
#[derive(Debug)]
pub struct ImageStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: Option<ElementImageSet>,
    stretch_image_to_widget_size: bool
}

/// <https://wiki.factorio.com/Types/LabelStyleSpecification>
#[derive(Debug)]
pub struct LabelStyleSpecification {
    base: StyleSpecificationBase,
    font: Option<String>, // Name of Font prototype
    font_color: Color,
    hovered_font_color: Color,
    clicked_font_color: Color,
    disabled_font_color: Color,
    rich_text_setting: RichTextSetting,
    single_line: bool,
    underlined: bool,
    rich_text_highlight_error_color: Color,
    rich_text_highlight_warning_color: Color,
    rich_text_highlight_ok_color: Color
}

/// <https://wiki.factorio.com/Types/LabelStyleSpecification#rich_text_setting>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RichTextSetting {
    Enabled,
    Disabled,
    Highlight,
}

/// <https://wiki.factorio.com/Types/LineStyleSpecification>
#[derive(Debug)]
pub struct LineStyleSpecification {
    base: StyleSpecificationBase,
    border: Option<BorderImageSet>
}

/// <https://wiki.factorio.com/Types/ListBoxStyleSpecification>
#[derive(Debug)]
pub struct ListBoxStyleSpecification {
    base: StyleSpecificationBase,
    item_style: Option<ButtonStyleSpecification>,
    scroll_pane_style: Option<ScrollPaneStyleSpecification>
}

/// <https://wiki.factorio.com/Types/ProgressBarStyleSpecification>
#[derive(Debug)]
pub struct ProgressBarStyleSpecification {
    base: StyleSpecificationBase,
    bar_width: u32,
    color: Color,
    other_colors: Vec<OtherColor>,
    bar: Option<ElementImageSet>,
    bar_background: Option<ElementImageSet>,
    font: Option<String>, // Name of Font Prototype
    font_color: Color,
    filled_font_color: Color,
    embed_text_in_bar: bool
}

/// <https://wiki.factorio.com/Types/ProgressBarStyleSpecification#other_colors>
#[derive(Debug)]
pub struct OtherColor {
    less_than: f64,
    color: Option<Color>,
    bar: Option<ElementImageSet>
}
