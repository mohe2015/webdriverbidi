use crate::remote::browser;
use crate::remote::script::{SerializationOptions, SharedReference};
use crate::remote::{JsInt, JsUint};
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
    pub locator_type: String,
    pub value: AccessibilityLocatorValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessibilityLocatorValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CssLocator {
    #[serde(rename = "type")]
    pub locator_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    pub locator_type: String,
    pub value: String,
    #[serde(rename = "ignoreCase", skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(rename = "matchType", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<InnerTextLocatorMatchType>,
    #[serde(rename = "maxDepth", skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<JsUint>,
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
    pub locator_type: String,
    pub value: String,
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
    pub method: String,
    pub params: ActivateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateParameters {
    pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshot {
    pub method: String,
    pub params: CaptureScreenshotParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshotParameters {
    pub context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<CaptureScreenshotParametersOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<ClipRectangle>,
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
    pub image_format_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<f32>, // 0.0..1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClipRectangle {
    BoxClipRectangle(BoxClipRectangle),
    ElementClipRectangle(ElementClipRectangle),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementClipRectangle {
    #[serde(rename = "type")]
    pub clip_rectangle_type: String,
    pub element: SharedReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoxClipRectangle {
    #[serde(rename = "type")]
    pub clip_rectangle_type: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
    pub method: String,
    pub params: CloseParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseParameters {
    pub context: BrowsingContext,
    #[serde(rename = "promptUnload", skip_serializing_if = "Option::is_none")]
    pub prompt_unload: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    pub method: String,
    pub params: CreateParameters,
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
    pub create_type: CreateType,
    #[serde(rename = "referenceContext", skip_serializing_if = "Option::is_none")]
    pub reference_context: Option<BrowsingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
    pub user_context: Option<browser::UserContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTree {
    pub method: String,
    pub params: GetTreeParameters,
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
    pub method: String,
    pub params: HandleUserPromptParameters,
}

impl HandleUserPrompt {
    pub fn new(params: HandleUserPromptParameters) -> Self {
        Self {
            method: "browsingContext.handleUserPrompt".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandleUserPromptParameters {
    pub context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_text: Option<String>,
}

impl HandleUserPromptParameters {
    pub fn new(context: BrowsingContext, accept: Option<bool>, user_text: Option<String>) -> Self {
        Self {
            context,
            accept,
            user_text,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodes {
    pub method: String,
    pub params: LocateNodesParameters,
}

impl LocateNodes {
    pub fn new(params: LocateNodesParameters) -> Self {
        Self {
            method: "browsingContext.locateNodes".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodesParameters {
    pub context: BrowsingContext,
    pub locator: Locator,
    #[serde(rename = "maxNodeCount", skip_serializing_if = "Option::is_none")]
    pub max_node_count: Option<JsUint>,
    #[serde(
        rename = "serializationOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "startNodes", skip_serializing_if = "Option::is_none")]
    pub start_nodes: Option<Vec<SharedReference>>,
}

impl LocateNodesParameters {
    pub fn new(
        context: BrowsingContext,
        locator: Locator,
        max_node_count: Option<JsUint>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    ) -> Self {
        Self {
            context,
            locator,
            max_node_count,
            serialization_options,
            start_nodes,
        }
    }
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
    pub method: String,
    pub params: PrintParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintParameters {
    pub context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<PrintMarginParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<PrintParametersOrientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PrintPageParameters>,
    #[serde(rename = "pageRanges", skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<Vec<JsUintOrText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>, // 0.1..2.0
    #[serde(rename = "shrinkToFit", skip_serializing_if = "Option::is_none")]
    pub shrink_to_fit: Option<bool>,
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
    pub bottom: Option<f32>, // 0.0..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>, // 0.0..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<f32>, // 0.0..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>, // 0.0..
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintPageParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>, // 0.0352..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>, // 0.0352..
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reload {
    pub method: String,
    pub params: ReloadParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReloadParameters {
    pub context: BrowsingContext,
    #[serde(rename = "ignoreCache", skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<ReadinessState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewport {
    pub method: String,
    pub params: SetViewportParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewportParameters {
    pub context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
    #[serde(rename = "devicePixelRatio", skip_serializing_if = "Option::is_none")]
    pub device_pixel_ratio: Option<f32>, // 0.0..
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewport {
    pub width: JsUint,
    pub height: JsUint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistory {
    pub method: String,
    pub params: TraverseHistoryParameters,
}

impl TraverseHistory {
    pub fn new(params: TraverseHistoryParameters) -> Self {
        Self {
            method: "browsingContext.traverseHistory".into(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistoryParameters {
    pub context: BrowsingContext,
    pub delta: JsInt,
}

impl TraverseHistoryParameters {
    pub fn new(context: String, delta: i64) -> Self {
        Self { context, delta }
    }
}
