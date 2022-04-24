use super::LocalisedString;
use super::{Color, Sprite, FileName, Position, SpriteSizeType, Sound};
use strum_macros::{EnumString, AsRefStr};

// FIXME?
// There is a big problem for recreating prototype API from public documentation which is intended
// for documenting how to use API, not reverse-engineer it. Many optional properties don't have
// default values, and it is not known (until further manual testing or documentation update)
// which of them are optional from API stand point and what their default values are.
// For now, I'll leave some properties that I think do have a default value or are mandatory
// without Option<> wrap.

/// <https://wiki.factorio.com/Types/StyleSpecification>
#[derive(Debug, Clone)]
pub enum StyleSpecification {
    ActivityBar(ActivityBarStyleSpecification),
    Camera(CameraStyleSpecification),
    Minimap(MinimapStyleSpecification),
    DropDown(Box<DropDownStyleSpecification>),
    Flow(FlowStyleSpecification),
    Frame(FrameStyleSpecification),
    Glow(GlowStyleSpecification),
    Graph(GraphStyleSpecification),
    HorizontalFlow(HorizontalFlowStyleSpecification),
    Image(ImageStyleSpecification),
    Label(LabelStyleSpecification),
    Line(LineStyleSpecification),
    ListBox(Box<ListBoxStyleSpecification>),
    ProgressBar(ProgressBarStyleSpecification),
    HorizontalScrollBar(HorizontalScrollBarStyleSpecification),
    VerticalScrollBar(VerticalScrollBarStyleSpecification),
    ScrollPane(ScrollPaneStyleSpecification),
    Slider(SliderStyleSpecification),
    DoubleSlider(DoubleSliderStyleSpecification),
    SpeechBubble(SpeechBubbleStyleSpecification),
    Button(ButtonStyleSpecification),
    TechnologySlot(Box<TechnologySlotStyleSpecification>),
    CheckBox(CheckBoxStyleSpecification),
    RadioButton(RadioButtonStyleSpecification),
    Switch(SwitchStyleSpecification),
    TabbedPane(TabbedPaneStyleSpecification),
    Table(Box<TableStyleSpecification>),
    Tab(TabStyleSpecification),
    TextBox(TextBoxStyleSpecification),
    VerticalFlow(VerticalFlowStyleSpecification)
}

/// <https://wiki.factorio.com/Types/StyleSpecification>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

/// <https://wiki.factorio.com/Types/StyleSpecification#vertical_align>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

/// <https://wiki.factorio.com/Types/StretchRule>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum StretchRule {
    On,
    Off,
    Auto,
    StretchAndExpand,
}

/// <https://wiki.factorio.com/Types/ActivityBarStyleSpecification>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum EmptyWidgetStyleSpecification {
    Camera(CameraStyleSpecification),
    Minimap(MinimapStyleSpecification)
}

/// <https://wiki.factorio.com/Types/EmptyWidgetStyleSpecification>
/// <https://wiki.factorio.com/Types/CameraStyleSpecification>
#[derive(Debug, Clone)]
pub struct CameraStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: ElementImageSet
}

/// <https://wiki.factorio.com/Types/EmptyWidgetStyleSpecification>
/// <https://wiki.factorio.com/Types/MinimapStyleSpecification>
#[derive(Debug, Clone)]
pub struct MinimapStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: ElementImageSet
}

/// <https://wiki.factorio.com/Types/ElementImageSet>
#[derive(Debug, Clone)]
pub struct ElementImageSet {
    base: ElementImageSetLayer,
    shadow: Option<ElementImageSetLayer>,
    glow: Option<ElementImageSetLayer>
}

/// <https://wiki.factorio.com/Types/ElementImageSetLayer>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DrawType {
    Inner,
    Outer,
}

/// <https://wiki.factorio.com/Types/ElementImageSetLayer#type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ElementImageSetLayerType {
    None,
    Composition,
}

/// <https://wiki.factorio.com/Types/DropDownStyleSpecification>
#[derive(Debug, Clone)]
pub struct DropDownStyleSpecification {
    base: StyleSpecificationBase,
    button_style: Option<ButtonStyleSpecification>,
    icon: Option<Sprite>,
    list_box_style: Option<ListBoxStyleSpecification>,
    selector_and_title_spacing: i16, // Default: 0 (unconfirmed)
    opened_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Types/FlowStyleSpecification>
#[derive(Debug, Clone)]
pub struct FlowStyleSpecification {
    base: StyleSpecificationBase,
    // Default 0 (unconfirmed)
    max_on_row: i32,
    horizontal_spacing: i32,
    vertical_spacing: i32
}

/// <https://wiki.factorio.com/Types/FrameStyleSpecification>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct GlowStyleSpecification {
    base: StyleSpecificationBase,
    image_set: Option<ElementImageSet>
}

/// <https://wiki.factorio.com/Types/GraphStyleSpecification>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct HorizontalFlowStyleSpecification {
    base: StyleSpecificationBase,
    horizontal_spacing: i32
}

/// <https://wiki.factorio.com/Types/VerticalFlowStyleSpecification>
#[derive(Debug, Clone)]
pub struct VerticalFlowStyleSpecification {
    base: StyleSpecificationBase,
    vertical_spacing: i32
}

/// <https://wiki.factorio.com/Types/ImageStyleSpecification>
#[derive(Debug, Clone)]
pub struct ImageStyleSpecification {
    base: StyleSpecificationBase,
    graphical_set: Option<ElementImageSet>,
    stretch_image_to_widget_size: bool
}

/// <https://wiki.factorio.com/Types/LabelStyleSpecification>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RichTextSetting {
    Enabled,
    Disabled,
    Highlight,
}

/// <https://wiki.factorio.com/Types/LineStyleSpecification>
#[derive(Debug, Clone)]
pub struct LineStyleSpecification {
    base: StyleSpecificationBase,
    border: Option<BorderImageSet>
}

/// <https://wiki.factorio.com/Types/ListBoxStyleSpecification>
#[derive(Debug, Clone)]
pub struct ListBoxStyleSpecification {
    base: StyleSpecificationBase,
    item_style: Option<ButtonStyleSpecification>,
    scroll_pane_style: Option<ScrollPaneStyleSpecification>
}

/// <https://wiki.factorio.com/Types/ProgressBarStyleSpecification>
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct OtherColor {
    less_than: f64,
    color: Option<Color>,
    bar: Option<ElementImageSet>
}

/// <https://wiki.factorio.com/Types/HorizontalScrollBarStyleSpecification>
#[derive(Debug, Clone)]
pub struct HorizontalScrollBarStyleSpecification {
    base: StyleSpecificationBase,
    background_graphical_set: Option<ElementImageSet>,
    thumb_button_style: Option<ButtonStyleSpecification>
}

/// <https://wiki.factorio.com/Types/VerticalScrollBarStyleSpecification>
#[derive(Debug, Clone)]
pub struct VerticalScrollBarStyleSpecification {
    base: StyleSpecificationBase,
    background_graphical_set: Option<ElementImageSet>,
    thumb_button_style: Option<ButtonStyleSpecification>
}

/// <https://wiki.factorio.com/Types/ScrollPaneStyleSpecification>
#[derive(Debug, Clone)]
pub struct ScrollPaneStyleSpecification {
    base: StyleSpecificationBase,
    vertical_flow_style: Option<VerticalFlowStyleSpecification>,
    horizontal_scrollbar_style: Option<HorizontalScrollBarStyleSpecification>,
    vertical_scrollbar_style: Option<VerticalScrollBarStyleSpecification>,
    graphical_set: Option<ElementImageSet>,
    background_graphical_set: Option<ElementImageSet>,
    extra_padding_when_activated: [i32; 4], // `extra_top_padding_when_activated`, `extra_bottom_padding_when_activated`, `extra_left_padding_when_activated` and `extra_right_padding_when_activated` // Default 0
    extra_margin_when_activated: [i32; 4], // `extra_top_margin_when_activated`, `extra_bottom_margin_when_activated`, `extra_left_margin_when_activated` and `extra_right_margin_when_activated` // Default 0
    dont_force_clipping_rect_for_contents: bool
}

/// <https://wiki.factorio.com/Types/SliderStyleSpecification>
#[derive(Debug, Clone)]
pub struct SliderStyleSpecification {
    base: StyleSpecificationBase,
    full_bar: Option<ElementImageSet>,
    full_bar_disabled: Option<ElementImageSet>,
    empty_bar: Option<ElementImageSet>,
    empty_bar_disabled: Option<ElementImageSet>,
    draw_notches: bool,
    notch: Option<ElementImageSet>,
    button: Option<ButtonStyleSpecification>,
    high_button: Option<ButtonStyleSpecification>
}

/// <https://wiki.factorio.com/Types/DoubleSliderStyleSpecification>
#[derive(Debug, Clone)]
pub struct DoubleSliderStyleSpecification {
    base: StyleSpecificationBase,
    full_bar: Option<ElementImageSet>,
    full_bar_disabled: Option<ElementImageSet>,
    empty_bar: Option<ElementImageSet>,
    empty_bar_disabled: Option<ElementImageSet>,
    draw_notches: bool,
    notch: Option<ElementImageSet>,
    button: Option<ButtonStyleSpecification>,
    high_button: Option<ButtonStyleSpecification>
}

/// <https://wiki.factorio.com/Types/SpeechBubbleStyleSpecification>
#[derive(Debug, Clone)]
pub struct SpeechBubbleStyleSpecification {
    base: StyleSpecificationBase,
    frame_style: Option<FrameStyleSpecification>,
    label_style: Option<LabelStyleSpecification>,
    arrow_graphical_set: Option<ElementImageSet>,
    close_color: Color,
    arrow_indent: f64,
    pass_through_mouse: bool
}

/// <https://wiki.factorio.com/Types/ButtonStyleSpecification>
#[derive(Debug, Clone)]
pub struct ButtonStyleSpecification {
    base: StyleSpecificationBase,
    default_graphical_set: Option<ElementImageSet>,
    hovered_graphical_set: Option<ElementImageSet>,
    clicked_graphical_set: Option<ElementImageSet>,
    disabled_graphical_set: Option<ElementImageSet>,
    selected_graphical_set: Option<ElementImageSet>,
    selected_hovered_graphical_set: Option<ElementImageSet>,
    selected_clicked_graphical_set: Option<ElementImageSet>,
    left_click_sound: Option<Sound>,
    font: Option<String>, // Name of Font prototype
    default_font_color: Color,
    hovered_font_color: Color,
    clicked_font_color: Color,
    disabled_font_color: Color,
    selected_font_color: Color,
    selected_hovered_font_color: Color,
    selected_clicked_font_color: Color,
    strikethrough_color: Color,
    pie_progress_color: Color,
    clicked_vertical_offset: u32,
    draw_shadow_under_picture: bool,
    draw_grayscale_picture: bool,
    icon_horizontal_align: HorizontalAlignment
}

/// <https://wiki.factorio.com/Types/TechnologySlotStyleSpecification>
#[derive(Debug, Clone)]
pub struct TechnologySlotStyleSpecification {
    base: StyleSpecificationBase,
    default_graphical_set: Option<ElementImageSet>,
    hovered_graphical_set: Option<ElementImageSet>,
    clicked_graphical_set: Option<ElementImageSet>,
    disabled_graphical_set: Option<ElementImageSet>,
    selected_graphical_set: Option<ElementImageSet>,
    selected_hovered_graphical_set: Option<ElementImageSet>,
    selected_clicked_graphical_set: Option<ElementImageSet>,
    left_click_sound: Option<Sound>,
    font: Option<String>, // Name of Font prototype
    default_font_color: Color,
    hovered_font_color: Color,
    clicked_font_color: Color,
    disabled_font_color: Color,
    selected_font_color: Color,
    selected_hovered_font_color: Color,
    selected_clicked_font_color: Color,
    strikethrough_color: Color,
    pie_progress_color: Color,
    clicked_vertical_offset: u32,
    draw_shadow_under_picture: bool,
    draw_grayscale_picture: bool,
    icon_horizontal_align: HorizontalAlignment,
    highlighted_graphical_set: Option<ElementImageSet>,
    default_background_shadow: Option<ElementImageSet>,
    level_band: Option<ElementImageSet>,
    hovered_level_band: Option<ElementImageSet>,
    level_offset_x: f32,
    level_offset_y: f32,
    level_band_width: u32,
    level_band_height: u32,
    level_font: String, // Name of Font prototype
    level_range_font: String, // Name of Font prototype
    level_font_color: Color,
    hovered_level_font_color: Color,
    level_range_font_color: Color,
    hovered_level_range_font_color: Color,
    level_range_band: Option<ElementImageSet>,
    hovered_level_range_band: Option<ElementImageSet>,
    level_range_offset_x: f32,
    level_range_offset_y: f32,
    ingredients_height: u32,
    default_ingredients_background: Option<ElementImageSet>,
    hovered_ingredients_background: Option<ElementImageSet>,
    clicked_ingredients_background: Option<ElementImageSet>,
    disabled_ingredients_background: Option<ElementImageSet>,
    highlighted_ingredients_background: Option<ElementImageSet>,
    ingredients_padding: u32,
    ingredient_icon_size: u32,
    ingredient_icon_overlap: u32,
    clicked_overlay: Option<ElementImageSet>,
    progress_bar_background: Option<ElementImageSet>,
    progress_bar: Option<ElementImageSet>,
    progress_bar_shadow: Option<ElementImageSet>,
    progress_bar_height: u32,
    progress_bar_color: Color
}

/// <https://wiki.factorio.com/Types/CheckBoxStyleSpecification>
#[derive(Debug, Clone)]
pub struct CheckBoxStyleSpecification {
    base: StyleSpecificationBase,
    default_graphical_set: Option<ElementImageSet>,
    hovered_graphical_set: Option<ElementImageSet>,
    clicked_graphical_set: Option<ElementImageSet>,
    disabled_graphical_set: Option<ElementImageSet>,
    selected_graphical_set: Option<ElementImageSet>,
    selected_hovered_graphical_set: Option<ElementImageSet>,
    selected_clicked_graphical_set: Option<ElementImageSet>,
    left_click_sound: Option<Sound>,
    font: Option<String>, // Nmae of Font prototype
    font_color: Color,
    checkmark: Option<Sprite>,
    disabled_checkmark: Option<Sprite>,
    intermediate_mark: Option<Sprite>,
    text_padding: u32
}

/// <https://wiki.factorio.com/Types/RadioButtonStyleSpecification>
#[derive(Debug, Clone)]
pub struct RadioButtonStyleSpecification {
    base: StyleSpecificationBase,
    default_graphical_set: Option<ElementImageSet>,
    hovered_graphical_set: Option<ElementImageSet>,
    clicked_graphical_set: Option<ElementImageSet>,
    disabled_graphical_set: Option<ElementImageSet>,
    selected_graphical_set: Option<ElementImageSet>,
    selected_hovered_graphical_set: Option<ElementImageSet>,
    selected_clicked_graphical_set: Option<ElementImageSet>,
    left_click_sound: Option<Sound>,
    font: Option<String>, // Name of Font prototype
    font_color: Color,
    text_padding: u32
}

/// <https://wiki.factorio.com/Types/SwitchStyleSpecification>
#[derive(Debug, Clone)]
pub struct SwitchStyleSpecification {
    base: StyleSpecificationBase,
    left_button_position: u32,
    middle_button_position: u32,
    right_button_position: u32,
    default_background: Option<Sprite>,
    hover_background: Option<Sprite>,
    disabled_background: Option<Sprite>,
    button: Option<ButtonStyleSpecification>,
    active_label: Option<LabelStyleSpecification>,
    inactive_label: Option<LabelStyleSpecification>
}

/// <https://wiki.factorio.com/Types/TabbedPaneStyleSpecification>
#[derive(Debug, Clone)]
pub struct TabbedPaneStyleSpecification {
    base: StyleSpecificationBase,
    vertical_spacing: u32,
    tab_content_frame: Option<FrameStyleSpecification>,
    tab_container: Option<HorizontalFlowStyleSpecification>
}

/// <https://wiki.factorio.com/Types/TableStyleSpecification>
#[derive(Debug, Clone)]
pub struct TableStyleSpecification {
    base: StyleSpecificationBase,
    horizontal_spacing: i32,
    vertical_spacing: i32,
    cell_padding: [i16; 4], // `top_cell_padding`, `right_cell_padding`, `bottom_cell_padding` and `left_cell_padding`
    apply_row_graphical_set_per_column: bool,
    column_graphical_set: Option<ElementImageSet>,
    default_row_graphical_set: Option<ElementImageSet>,
    even_row_graphical_set: Option<ElementImageSet>,
    odd_row_graphical_set: Option<ElementImageSet>,
    hovered_graphical_set: Option<ElementImageSet>,
    clicked_graphical_set: Option<ElementImageSet>,
    selected_graphical_set: Option<ElementImageSet>,
    selected_hovered_graphical_set: Option<ElementImageSet>,
    selected_clicked_graphical_set: Option<ElementImageSet>,
    background_graphical_set: Option<ElementImageSet>,
    column_alignments: Vec<ColumnAlignment>,
    column_widths: Vec<ColumnWidth>,
    hovered_row_color: Color,
    selected_row_color: Color,
    vertical_line_color: Color,
    horizontal_line_color: Color,
    column_ordering_ascending_button_style: Option<ButtonStyleSpecification>,
    column_ordering_descending_button_style: Option<ButtonStyleSpecification>,
    inactive_column_ordering_ascending_button_style: Option<ButtonStyleSpecification>,
    inactive_column_ordering_descending_button_style: Option<ButtonStyleSpecification>,
    border: Option<BorderImageSet>
}

/// <https://wiki.factorio.com/Types/TableStyleSpecification#column_alignments>
#[derive(Debug, Clone)]
pub struct ColumnAlignment {
    column: u32,
    alignment: ColumnAlignmentStr
}

/// <https://wiki.factorio.com/Types/TableStyleSpecification#column_widths>
#[derive(Debug, Clone)]
pub struct ColumnWidth {
    column: u32,
    width: [i32; 2], // `minimal_width` and `maximal_width` // Default: 0
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ColumnAlignmentStr {
    Center,
    Left,
    Right,
    TopLeft,
    MiddleLeft,
    BottomLeft,
    TopCenter,
    MiddleCenter,
    BottomCenter,
    TopRight,
    MiddleRight,
    BottomRight
}

/// <https://wiki.factorio.com/Types/TabStyleSpecification>
#[derive(Debug, Clone)]
pub struct TabStyleSpecification {
    base: StyleSpecificationBase,
    font: Option<String>, // Name of Font prototype
    badge_font: Option<String>, // Name of Font prototype
    badge_horizontal_spacing: i16,
    default_font_color: Color,
    selected_font_color: Color,
    disabled_font_color: Color,
    default_badge_font_color: Color,
    selected_badge_font_color: Color,
    disabled_badge_font_color: Color,
    default_graphical_set: Option<ElementImageSet>,
    selected_graphical_set: Option<ElementImageSet>,
    hover_graphical_set: Option<ElementImageSet>,
    press_graphical_set: Option<ElementImageSet>,
    disabled_graphical_set: Option<ElementImageSet>,
    override_graphics_on_edges: bool,
    left_edge_selected_graphical_set: Option<ElementImageSet>,
    right_edge_selected_graphical_set: Option<ElementImageSet>,
    default_badge_graphical_set: Option<ElementImageSet>,
    selected_badge_graphical_set: Option<ElementImageSet>,
    hover_badge_graphical_set: Option<ElementImageSet>,
    press_badge_graphical_set: Option<ElementImageSet>,
    disabled_badge_graphical_set: Option<ElementImageSet>,
    draw_grayscale_picture: bool,
    left_click_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Types/TextBoxStyleSpecification>
#[derive(Debug, Clone)]
pub struct TextBoxStyleSpecification {
    base: StyleSpecificationBase,
    font: Option<String>, // Name of Font prototype
    font_color: Color,
    disabled_font_color: Color,
    selection_background_color: Color,
    default_background: Option<ElementImageSet>,
    active_background: Option<ElementImageSet>,
    disabled_background: Option<ElementImageSet>,
    rich_text_setting: RichTextSetting,
    rich_text_highlight_error_color: Color,
    rich_text_highlight_warning_color: Color,
    rich_text_highlight_ok_color: Color,
    selected_rich_text_highlight_error_color: Color,
    selected_rich_text_highlight_warning_color: Color,
    selected_rich_text_highlight_ok_color: Color
}
