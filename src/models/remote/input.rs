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
    element_origin_type: String,
    element: SharedReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActions {
    method: String,
    params: PerformActionsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActionsParameters {
    context: BrowsingContext,
    actions: Vec<SourceActions>,
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
    none_source_actions_type: String,
    id: String,
    actions: Vec<NoneSourceAction>,
}

pub type NoneSourceAction = PauseAction;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeySourceActions {
    #[serde(rename = "type")]
    key_source_actions_type: String,
    id: String,
    actions: Vec<KeySourceAction>,
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
    pointer_source_actions_type: String,
    id: String,
    actions: Vec<PointerSourceAction>,
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
    pointer_type: Option<PointerType>,
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
    wheel_source_actions_type: String,
    id: String,
    actions: Vec<WheelSourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WheelSourceAction {
    PauseAction(PauseAction),
    WheelScrollAction(WheelScrollAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PauseAction {
    #[serde(rename = "type")]
    pause_action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<JsUint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDownAction {
    #[serde(rename = "type")]
    key_down_action_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyUpAction {
    #[serde(rename = "type")]
    key_up_action_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerUpAction {
    #[serde(rename = "type")]
    pointer_up_action_type: String,
    button: JsUint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerDownAction {
    #[serde(rename = "type")]
    pointer_down_action_type: String,
    button: JsUint,
    #[serde(flatten)]
    pointer_common_properties: PointerCommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerMoveAction {
    #[serde(rename = "type")]
    pointer_move_action_type: String,
    x: JsInt,
    y: JsInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Origin>,
    #[serde(flatten)]
    pointer_common_properties: PointerCommonProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelScrollAction {
    #[serde(rename = "type")]
    wheel_scroll_action_type: String,
    x: JsInt,
    y: JsInt,
    #[serde(rename = "deltaX")]
    delta_x: JsInt,
    #[serde(rename = "deltaY")]
    delta_y: JsInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Origin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerCommonProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pressure: Option<f64>,
    #[serde(rename = "tangentialPressure", skip_serializing_if = "Option::is_none")]
    tangential_pressure: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twist: Option<JsUint>,
    #[serde(rename = "altitudeAngle", skip_serializing_if = "Option::is_none")]
    altitude_angle: Option<f64>,
    #[serde(rename = "azimuthAngle", skip_serializing_if = "Option::is_none")]
    azimuth_angle: Option<f64>,
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
    method: String,
    params: ReleaseActionsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActionsParameters {
    context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFiles {
    method: String,
    params: SetFilesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFilesParameters {
    context: BrowsingContext,
    element: SharedReference,
    files: Vec<String>,
}
