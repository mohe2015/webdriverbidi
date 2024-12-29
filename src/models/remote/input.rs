use crate::models::remote::browsing_context::BrowsingContext;
use crate::models::remote::script::SharedReference;
use crate::models::remote::{JsInt, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum InputCommand {
    PerformActions(PerformActions),
    ReleaseActions(ReleaseActions),
    SetFiles(SetFiles),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementOrigin {
    #[serde(rename = "type")]
    pub element_origin_type: String,
    pub element: SharedReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActions {
    pub method: String,
    pub params: PerformActionsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActionsParameters {
    pub context: BrowsingContext,
    pub actions: Vec<SourceActions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SourceActions {
    NoneSourceActions(NoneSourceActions),
    KeySourceActions(KeySourceActions),
    PointerSourceActions(PointerSourceActions),
    WheelSourceActions(WheelSourceActions),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoneSourceActions {
    #[serde(rename = "type")]
    pub none_source_actions_type: String,
    pub id: String,
    pub actions: Vec<NoneSourceAction>,
}

pub type NoneSourceAction = PauseAction;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeySourceActions {
    #[serde(rename = "type")]
    pub key_source_actions_type: String,
    pub id: String,
    pub actions: Vec<KeySourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeySourceAction {
    PauseAction(PauseAction),
    KeyDownAction(KeyDownAction),
    KeyUpAction(KeyUpAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerSourceActions {
    #[serde(rename = "type")]
    pub pointer_source_actions_type: String,
    pub id: String,
    pub actions: Vec<PointerSourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerParameters {
    #[serde(rename = "pointerType", skip_serializing_if = "Option::is_none")]
    pub pointer_type: Option<PointerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PointerSourceAction {
    PauseAction(PauseAction),
    PointerDownAction(PointerDownAction),
    PointerUpAction(PointerUpAction),
    PointerMoveAction(PointerMoveAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelSourceActions {
    #[serde(rename = "type")]
    pub wheel_source_actions_type: String,
    pub id: String,
    pub actions: Vec<WheelSourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WheelSourceAction {
    PauseAction(PauseAction),
    WheelScrollAction(WheelScrollAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PauseAction {
    #[serde(rename = "type")]
    pub pause_action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<JsUint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDownAction {
    #[serde(rename = "type")]
    pub key_down_action_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyUpAction {
    #[serde(rename = "type")]
    pub key_up_action_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerUpAction {
    #[serde(rename = "type")]
    pub pointer_up_action_type: String,
    pub button: JsUint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerDownAction {
    #[serde(rename = "type")]
    pub pointer_down_action_type: String,
    pub button: JsUint,
    #[serde(flatten)]
    pub pointer_common_properties: PointerCommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerMoveAction {
    #[serde(rename = "type")]
    pub pointer_move_action_type: String,
    pub x: JsInt,
    pub y: JsInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    #[serde(flatten)]
    pub pointer_common_properties: PointerCommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelScrollAction {
    #[serde(rename = "type")]
    pub wheel_scroll_action_type: String,
    pub x: JsInt,
    pub y: JsInt,
    #[serde(rename = "deltaX")]
    pub delta_x: JsInt,
    #[serde(rename = "deltaY")]
    pub delta_y: JsInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerCommonProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure: Option<f64>,
    #[serde(rename = "tangentialPressure", skip_serializing_if = "Option::is_none")]
    pub tangential_pressure: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twist: Option<JsUint>,
    #[serde(rename = "altitudeAngle", skip_serializing_if = "Option::is_none")]
    pub altitude_angle: Option<f64>,
    #[serde(rename = "azimuthAngle", skip_serializing_if = "Option::is_none")]
    pub azimuth_angle: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "viewport")]
    Viewport,
    #[serde(rename = "pointer")]
    Pointer,
    ElementOrigin(ElementOrigin),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActions {
    pub method: String,
    pub params: ReleaseActionsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActionsParameters {
    pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFiles {
    pub method: String,
    pub params: SetFilesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFilesParameters {
    pub context: BrowsingContext,
    pub element: SharedReference,
    pub files: Vec<String>,
}
