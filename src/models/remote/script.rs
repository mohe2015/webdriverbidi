use crate::models::remote::browsing_context::BrowsingContext;
use crate::models::remote::{Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptCommand {
    AddPreloadScript(AddPreloadScript),
    CallFunction(CallFunction),
    Disown(Disown),
    Evaluate(Evaluate),
    GetRealms(GetRealms),
    RemovePreloadScript(RemovePreloadScript),
}

pub type Channel = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelValue {
    #[serde(rename = "type")]
    pub channel_value_type: String,
    pub value: ChannelProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelProperties {
    pub channel: Channel,
    #[serde(
        rename = "serializationOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<ResultOwnership>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvaluateResult {
    EvaluateResultSuccess(EvaluateResultSuccess),
    EvaluateResultException(EvaluateResultException),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
    #[serde(rename = "type")]
    pub evaluate_result_success_type: String,
    pub result: RemoteValue,
    pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultException {
    #[serde(rename = "type")]
    pub evaluate_result_exception_type: String,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: ExceptionDetails,
    pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionDetails {
    #[serde(rename = "columnNumber")]
    pub column_number: JsUint,
    pub exception: RemoteValue,
    #[serde(rename = "lineNumber")]
    pub line_number: JsUint,
    #[serde(rename = "stackTrace")]
    pub stack_trace: StackTrace,
    pub text: String,
}

pub type Handle = String;
pub type InternalId = String;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayLocalValue {
    #[serde(rename = "type")]
    pub array_local_value_type: String,
    pub value: ListLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLocalValue {
    #[serde(rename = "type")]
    pub date_local_value_type: String,
    pub value: String,
}

pub type MappingLocalValue = Vec<(LocalValueOrText, LocalValue)>;

#[derive(Debug, Serialize, Deserialize)]
pub enum LocalValueOrText {
    LocalValue(LocalValue),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapLocalValue {
    #[serde(rename = "type")]
    pub map_local_value_type: String,
    pub value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLocalValue {
    #[serde(rename = "type")]
    pub object_local_value_type: String,
    pub value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpValue {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpLocalValue {
    #[serde(rename = "type")]
    pub regexp_local_value_type: String,
    pub value: RegExpValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetLocalValue {
    #[serde(rename = "type")]
    pub set_local_value_type: String,
    pub value: ListLocalValue,
}

pub type PreloadScript = String;
pub type Realm = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum PrimitiveProtocolValue {
    UndefinedValue(UndefinedValue),
    NullValue(NullValue),
    StringValue(StringValue),
    NumberValue(NumberValue),
    BooleanValue(BooleanValue),
    BigIntValue(BigIntValue),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndefinedValue {
    #[serde(rename = "type")]
    pub undefined_value_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullValue {
    #[serde(rename = "type")]
    pub null_value_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub string_value_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialNumber {
    NaN,
    #[serde(rename = "-0")]
    NegativeZero,
    Infinity,
    #[serde(rename = "-Infinity")]
    NegativeInfinity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberValue {
    #[serde(rename = "type")]
    pub number_value_type: String,
    pub value: NumberOrSpecialNumber,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NumberOrSpecialNumber {
    Number(f64),
    SpecialNumber(SpecialNumber),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
    #[serde(rename = "type")]
    pub boolean_value_type: String,
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigIntValue {
    #[serde(rename = "type")]
    pub bigint_value_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteReference {
    SharedReference(SharedReference),
    RemoteObjectReference(RemoteObjectReference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedReference {
    #[serde(rename = "sharedId")]
    pub shared_id: SharedId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteObjectReference {
    pub handle: Handle,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    pub shared_id: Option<SharedId>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteValueOrText {
    RemoteValue(RemoteValue),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolRemoteValue {
    #[serde(rename = "type")]
    pub symbol_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayRemoteValue {
    #[serde(rename = "type")]
    pub array_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRemoteValue {
    #[serde(rename = "type")]
    pub object_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
    #[serde(rename = "type")]
    pub function_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(flatten)]
    pub reg_exp_local_value: RegExpLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRemoteValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(flatten)]
    pub date_local_value: DateLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapRemoteValue {
    #[serde(rename = "type")]
    pub map_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    pub value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetRemoteValue {
    #[serde(rename = "type")]
    pub set_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
    #[serde(rename = "type")]
    pub weak_map_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
    #[serde(rename = "type")]
    pub weak_set_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
    #[serde(rename = "type")]
    pub generator_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
    #[serde(rename = "type")]
    pub error_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
    #[serde(rename = "type")]
    pub proxy_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
    #[serde(rename = "type")]
    pub promise_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
    #[serde(rename = "type")]
    pub typed_array_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
    #[serde(rename = "type")]
    pub array_buffer_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListRemoteValue {
    #[serde(rename = "type")]
    pub node_list_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTMLCollectionRemoteValue {
    #[serde(rename = "type")]
    pub html_collection_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRemoteValue {
    #[serde(rename = "type")]
    pub node_remote_value_type: String,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    pub shared_id: Option<SharedId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<NodeProperties>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeProperties {
    #[serde(rename = "nodeType")]
    pub node_type: JsUint,
    #[serde(rename = "childNodeCount")]
    pub child_node_count: JsUint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<NodeRemoteValue>>,
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<NodePropertiesMode>,
    #[serde(rename = "namespaceURI", skip_serializing_if = "Option::is_none")]
    pub namespace_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeValue")]
    pub node_value: Option<String>,
    // Recursive type
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(rename = "shadowRoot")]
    // pub shadow_root: Option<Option<NodeRemoteValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodePropertiesMode {
    Open,
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyRemoteValue {
    #[serde(rename = "type")]
    pub window_proxy_remote_value_type: String,
    pub value: WindowProxyProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Handle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyProperties {
    pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResultOwnership {
    Root,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializationOptions {
    #[serde(rename = "maxDomDepth", skip_serializing_if = "Option::is_none")]
    pub max_dom_depth: Option<JsUint>,
    #[serde(rename = "maxObjectDepth", skip_serializing_if = "Option::is_none")]
    pub max_object_depth: Option<JsUint>,
    #[serde(rename = "includeShadowTree", skip_serializing_if = "Option::is_none")]
    pub include_shadow_tree: Option<IncludeShadowTree>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IncludeShadowTree {
    None,
    Open,
    All,
}

pub type SharedId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
    #[serde(rename = "columnNumber")]
    pub column_number: JsUint,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "lineNumber")]
    pub line_number: JsUint,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTrace {
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<StackFrame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmTarget {
    pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextTarget {
    pub context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
    ContextTarget(ContextTarget),
    RealmTarget(RealmTarget),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScript {
    pub method: String,
    pub params: AddPreloadScriptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptParameters {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<ChannelValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disown {
    pub method: String,
    pub params: DisownParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisownParameters {
    pub handles: Vec<Handle>,
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunction {
    pub method: String,
    pub params: CallFunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunctionParameters {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
    pub target: Target,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<LocalValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultOwnership")]
    pub result_ownership: Option<ResultOwnership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub this: Option<LocalValue>,
    #[serde(rename = "userActivation", skip_serializing_if = "Option::is_none")]
    pub user_activation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
    pub method: String,
    pub params: EvaluateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateParameters {
    pub expression: String,
    pub target: Target,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultOwnership")]
    pub result_ownership: Option<ResultOwnership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "userActivation", skip_serializing_if = "Option::is_none")]
    pub user_activation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealms {
    pub method: String,
    pub params: GetRealmsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<BrowsingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub realm_type: Option<RealmType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScript {
    pub method: String,
    pub params: RemovePreloadScriptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScriptParameters {
    pub script: PreloadScript,
}
