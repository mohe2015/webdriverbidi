use crate::models::remote::browser;
use crate::models::remote::script::{SerializationOptions, SharedReference};
use crate::models::remote::{JsInt, JsUint};
use serde::{Deserialize, Serialize};

pub type BrowsingContext = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum BrowsingContextCommand {
    Activate(Activate),
    CaptureScreenshot(CaptureScreenshot),
    Close(Close),
    Create(Create),
    GetTree(GetTree),
    HandleUserPrompt(HandleUserPrompt),
    LocateNodes(LocateNodes),
    Navigate(Navigate),
    Print(Print),
    Reload(Reload),
    SetViewport(SetViewport),
    TraverseHistory(TraverseHistory),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Locator {
    AccessibilityLocator(AccessibilityLocator),
    CssLocator(CssLocator),
    InnerTextLocator(InnerTextLocator),
    XPathLocator(XPathLocator),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: AccessibilityLocatorValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocatorValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CssLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: String,
    #[serde(rename = "ignoreCase", skip_serializing_if = "Option::is_none")]
    ignore_case: Option<bool>,
    #[serde(rename = "matchType", skip_serializing_if = "Option::is_none")]
    match_type: Option<InnerTextLocatorMatchType>,
    #[serde(rename = "maxDepth", skip_serializing_if = "Option::is_none")]
    max_depth: Option<JsUint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InnerTextLocatorMatchType {
    Full,
    Partial,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XPathLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadinessState {
    Complete,
    Interactive,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserPromptType {
    Alert,
    BeforeUnload,
    Confirm,
    Prompt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activate {
    method: String,
    params: ActivateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateParameters {
    context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshot {
    method: String,
    params: CaptureScreenshotParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshotParameters {
    context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<CaptureScreenshotParametersOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ImageFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clip: Option<ClipRectangle>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureScreenshotParametersOrigin {
    Document,
    Viewport,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageFormat {
    #[serde(rename = "type")]
    image_format_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<f32>, // 0.0..1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClipRectangle {
    BoxClipRectangle(BoxClipRectangle),
    ElementClipRectangle(ElementClipRectangle),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementClipRectangle {
    #[serde(rename = "type")]
    clip_rectangle_type: String,
    element: SharedReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoxClipRectangle {
    #[serde(rename = "type")]
    clip_rectangle_type: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
    method: String,
    params: CloseParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseParameters {
    context: BrowsingContext,
    #[serde(rename = "promptUnload", skip_serializing_if = "Option::is_none")]
    prompt_unload: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    method: String,
    params: CreateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateType {
    Tab,
    Window,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParameters {
    #[serde(rename = "type")]
    create_type: CreateType,
    #[serde(rename = "referenceContext", skip_serializing_if = "Option::is_none")]
    reference_context: Option<BrowsingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<bool>,
    #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
    user_context: Option<browser::UserContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTree {
    method: String,
    params: GetTreeParameters,
}

impl GetTree {
    pub fn new(params: GetTreeParameters) -> Self {
        Self {
            method: "browsingContext.getTree".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTreeParameters {
    #[serde(rename = "maxDepth", skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<BrowsingContext>,
}

impl GetTreeParameters {
    pub fn new(max_depth: Option<JsUint>, root: Option<BrowsingContext>) -> Self {
        Self { max_depth, root }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandleUserPrompt {
    method: String,
    params: HandleUserPromptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandleUserPromptParameters {
    context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodes {
    method: String,
    params: LocateNodesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodesParameters {
    context: BrowsingContext,
    locator: Locator,
    #[serde(rename = "maxNodeCount", skip_serializing_if = "Option::is_none")]
    max_node_count: Option<JsUint>,
    #[serde(
        rename = "serializationOptions",
        skip_serializing_if = "Option::is_none"
    )]
    serialization_options: Option<SerializationOptions>,
    #[serde(rename = "startNodes", skip_serializing_if = "Option::is_none")]
    start_nodes: Option<Vec<SharedReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Navigate {
    pub method: String, // "browsingContext.navigate"
    pub params: NavigateParameters,
}

impl Navigate {
    pub fn new(params: NavigateParameters) -> Self {
        Self {
            method: "browsingContext.navigate".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigateParameters {
    pub context: BrowsingContext,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<ReadinessState>,
}

impl NavigateParameters {
    pub fn new(context: BrowsingContext, url: String, wait: Option<ReadinessState>) -> Self {
        Self { context, url, wait }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
    method: String,
    params: PrintParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintParameters {
    context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    margin: Option<PrintMarginParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<PrintParametersOrientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<PrintPageParameters>,
    #[serde(rename = "pageRanges", skip_serializing_if = "Option::is_none")]
    page_ranges: Option<Vec<JsUintOrText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f32>, // 0.1..2.0
    #[serde(rename = "shrinkToFit", skip_serializing_if = "Option::is_none")]
    shrink_to_fit: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrintParametersOrientation {
    Landscape,
    Portrait,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JsUintOrText {
    JsUint(JsUint),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintMarginParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<f32>, // 0.0..
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<f32>, // 0.0..
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<f32>, // 0.0..
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<f32>, // 0.0..
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintPageParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f32>, // 0.0352..
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f32>, // 0.0352..
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reload {
    method: String,
    params: ReloadParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReloadParameters {
    context: BrowsingContext,
    #[serde(rename = "ignoreCache", skip_serializing_if = "Option::is_none")]
    ignore_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<ReadinessState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewport {
    method: String,
    params: SetViewportParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewportParameters {
    context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewport: Option<Viewport>,
    #[serde(rename = "devicePixelRatio", skip_serializing_if = "Option::is_none")]
    device_pixel_ratio: Option<f32>, // 0.0..
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewport {
    width: JsUint,
    height: JsUint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistory {
    method: String,
    params: TraverseHistoryParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistoryParameters {
    context: BrowsingContext,
    delta: JsInt,
}
