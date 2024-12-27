use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::local::{browsing_context, Extensible, JsUint};

#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptResult {
    AddPreloadScriptResult(AddPreloadScriptResult),
    EvaluateResult(EvaluateResult),
    GetRealmsResult(GetRealmsResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptEvent {
    Message(Message),
    RealmCreated(RealmCreated),
    RealmDestroyed(RealmDestroyed),
}

pub type Channel = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelValue {
    #[serde(rename = "type")]
    value_type: String,
    value: ChannelProperties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelProperties {
    channel: Channel,
    #[serde(
        rename = "serializationOptions",
        skip_serializing_if = "Option::is_none"
    )]
    serialization_options: Option<SerializationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership: Option<ResultOwnership>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EvaluateResult {
    EvaluateResultSuccess(EvaluateResultSuccess),
    EvaluateResultException(EvaluateResultException),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateResultSuccess {
    #[serde(rename = "type")]
    result_type: String,
    result: RemoteValue,
    realm: Realm,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateResultException {
    #[serde(rename = "type")]
    result_type: String,
    #[serde(rename = "exceptionDetails")]
    exception_details: ExceptionDetails,
    realm: Realm,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExceptionDetails {
    #[serde(rename = "columnNumber")]
    column_number: JsUint,
    exception: RemoteValue,
    #[serde(rename = "lineNumber")]
    line_number: JsUint,
    #[serde(rename = "stackTrace")]
    stack_trace: StackTrace,
    text: String,
}

pub type Handle = String;
pub type InternalId = String;

#[derive(Serialize, Deserialize, Debug)]
pub enum LocalValue {
    RemoteReference(RemoteReference),
    PrimitiveProtocolValue(PrimitiveProtocolValue),
    ChannelValue(ChannelValue),
    ArrayLocalValue(ArrayLocalValue),
    DateLocalValue(DateLocalValue),
    MapLocalValue(MapLocalValue),
    ObjectLocalValue(ObjectLocalValue),
    RegExpLocalValue(RegExpLocalValue),
    SetLocalValue(SetLocalValue),
}

pub type ListLocalValue = Vec<LocalValue>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayLocalValue {
    #[serde(rename = "type")]
    value_type: String,
    value: ListLocalValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateLocalValue {
    #[serde(rename = "type")]
    value_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingLocalValue(pub Vec<(LocalValueOrText, LocalValue)>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalValueOrText {
    LocalValue(LocalValue),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapLocalValue {
    #[serde(rename = "type")]
    value_type: String,
    value: MappingLocalValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectLocalValue {
    #[serde(rename = "type")]
    value_type: String,
    value: MappingLocalValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegExpValue {
    pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegExpLocalValue {
    #[serde(rename = "type")]
    value_type: String,
    value: RegExpValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetLocalValue {
    #[serde(rename = "type")]
    value_type: String,
    value: ListLocalValue,
}

pub type PreloadScript = String;
pub type Realm = String;

#[derive(Serialize, Deserialize, Debug)]
pub enum PrimitiveProtocolValue {
    UndefinedValue(UndefinedValue),
    NullValue(NullValue),
    StringValue(StringValue),
    NumberValue(NumberValue),
    BooleanValue(BooleanValue),
    BigIntValue(BigIntValue),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UndefinedValue {
    #[serde(rename = "type")]
    value_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NullValue {
    #[serde(rename = "type")]
    value_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringValue {
    #[serde(rename = "type")]
    value_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpecialNumber {
    NaN,
    #[serde(rename = "-0")]
    NegativeZero,
    Infinity,
    #[serde(rename = "-Infinity")]
    NegativeInfinity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumberValue {
    #[serde(rename = "type")]
    value_type: String,
    value: NumberOrSpecialNumber,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NumberOrSpecialNumber {
    Number(f64),
    SpecialNumber(SpecialNumber),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BooleanValue {
    #[serde(rename = "type")]
    value_type: String,
    value: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BigIntValue {
    #[serde(rename = "type")]
    value_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RealmInfo {
    WindowRealmInfo(WindowRealmInfo),
    DedicatedWorkerRealmInfo(DedicatedWorkerRealmInfo),
    SharedWorkerRealmInfo(SharedWorkerRealmInfo),
    ServiceWorkerRealmInfo(ServiceWorkerRealmInfo),
    WorkerRealmInfo(WorkerRealmInfo),
    PaintWorkletRealmInfo(PaintWorkletRealmInfo),
    AudioWorkletRealmInfo(AudioWorkletRealmInfo),
    WorkletRealmInfo(WorkletRealmInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseRealmInfo {
    realm: Realm,
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
    context: browsing_context::BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DedicatedWorkerRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
    owners: Vec<Realm>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedWorkerRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceWorkerRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaintWorkletRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioWorkletRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkletRealmInfo {
    #[serde(flatten)]
    base: BaseRealmInfo,
    #[serde(rename = "type")]
    realm_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RealmType {
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "dedicated-worker")]
    DedicatedWorker,
    #[serde(rename = "shared-worker")]
    SharedWorker,
    #[serde(rename = "service-worker")]
    ServiceWorker,
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "paint-worklet")]
    PaintWorklet,
    #[serde(rename = "audio-worklet")]
    AudioWorklet,
    #[serde(rename = "worklet")]
    Worklet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RemoteReference {
    SharedReference(SharedReference),
    RemoteObjectReference(RemoteObjectReference),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedReference {
    #[serde(rename = "sharedId")]
    shared_id: SharedId,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(flatten)]
    extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteObjectReference {
    handle: Handle,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    shared_id: Option<SharedId>,
    #[serde(flatten)]
    extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RemoteValue {
    PrimitiveProtocolValue(PrimitiveProtocolValue),
    SymbolRemoteValue(SymbolRemoteValue),
    ArrayRemoteValue(ArrayRemoteValue),
    ObjectRemoteValue(ObjectRemoteValue),
    FunctionRemoteValue(FunctionRemoteValue),
    RegExpRemoteValue(RegExpRemoteValue),
    DateRemoteValue(DateRemoteValue),
    MapRemoteValue(MapRemoteValue),
    SetRemoteValue(SetRemoteValue),
    WeakMapRemoteValue(WeakMapRemoteValue),
    WeakSetRemoteValue(WeakSetRemoteValue),
    GeneratorRemoteValue(GeneratorRemoteValue),
    ErrorRemoteValue(ErrorRemoteValue),
    ProxyRemoteValue(ProxyRemoteValue),
    PromiseRemoteValue(PromiseRemoteValue),
    TypedArrayRemoteValue(TypedArrayRemoteValue),
    ArrayBufferRemoteValue(ArrayBufferRemoteValue),
    NodeListRemoteValue(NodeListRemoteValue),
    HTMLCollectionRemoteValue(HTMLCollectionRemoteValue),
    NodeRemoteValue(NodeRemoteValue),
    WindowProxyRemoteValue(WindowProxyRemoteValue),
}

pub type ListRemoteValue = Vec<RemoteValue>;

pub type MappingRemoteValue = Vec<(RemoteValueOrText, RemoteValue)>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RemoteValueOrText {
    RemoteValue(RemoteValue),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<MappingRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegExpRemoteValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(rename = "type")]
    value_type: String,
    value: RegExpValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateRemoteValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(rename = "type")]
    value_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<MappingRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeakMapRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeakSetRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratorRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromiseRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypedArrayRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayBufferRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeListRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HTMLCollectionRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    shared_id: Option<SharedId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Box<NodeProperties>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeProperties {
    #[serde(rename = "nodeType")]
    node_type: JsUint,
    #[serde(rename = "childNodeCount")]
    child_node_count: JsUint,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<NodeRemoteValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "localName")]
    local_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mode")]
    mode: Option<NodePropertiesMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "namespaceURI")]
    namespace_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeValue")]
    node_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shadowRoot")]
    shadow_root: Option<NodeRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // Match JSON values ("open" / "closed")
pub enum NodePropertiesMode {
    Open,
    Closed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowProxyRemoteValue {
    #[serde(rename = "type")]
    value_type: String,
    value: WindowProxyProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowProxyProperties {
    context: browsing_context::BrowsingContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResultOwnership {
    Root,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializationOptions {
    #[serde(rename = "maxDomDepth", skip_serializing_if = "Option::is_none")]
    max_dom_depth: Option<JsUint>,
    #[serde(rename = "maxObjectDepth", skip_serializing_if = "Option::is_none")]
    max_object_depth: Option<JsUint>,
    #[serde(rename = "includeShadowTree", skip_serializing_if = "Option::is_none")]
    include_shadow_tree: Option<IncludeShadowTree>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum IncludeShadowTree {
    None,
    Open,
    All,
}

pub type SharedId = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct StackFrame {
    #[serde(rename = "columnNumber")]
    column_number: JsUint,
    #[serde(rename = "functionName")]
    function_name: String,
    #[serde(rename = "lineNumber")]
    line_number: JsUint,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StackTrace {
    #[serde(rename = "callFrames")]
    call_frames: Vec<StackFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    realm: Realm,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<browsing_context::BrowsingContext>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddPreloadScriptResult {
    script: PreloadScript,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRealmsResult {
    realms: Vec<RealmInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    method: String,
    params: MessageParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageParameters {
    channel: Channel,
    data: RemoteValue,
    source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealmCreated {
    method: String,
    params: RealmInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealmDestroyed {
    method: String,
    params: RealmDestroyedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RealmDestroyedParameters {
    realm: Realm,
}
