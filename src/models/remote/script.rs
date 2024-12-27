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
    channel_value_type: String,
    value: ChannelProperties,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum EvaluateResult {
    EvaluateResultSuccess(EvaluateResultSuccess),
    EvaluateResultException(EvaluateResultException),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
    #[serde(rename = "type")]
    evaluate_result_success_type: String,
    result: RemoteValue,
    realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultException {
    #[serde(rename = "type")]
    evaluate_result_exception_type: String,
    #[serde(rename = "exceptionDetails")]
    exception_details: ExceptionDetails,
    realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
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
    array_local_value_type: String,
    value: ListLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLocalValue {
    #[serde(rename = "type")]
    date_local_value_type: String,
    value: String,
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
    map_local_value_type: String,
    value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLocalValue {
    #[serde(rename = "type")]
    object_local_value_type: String,
    value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpValue {
    pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpLocalValue {
    #[serde(rename = "type")]
    regexp_local_value_type: String,
    value: RegExpValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetLocalValue {
    #[serde(rename = "type")]
    set_local_value_type: String,
    value: ListLocalValue,
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
    undefined_value_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullValue {
    #[serde(rename = "type")]
    null_value_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    string_value_type: String,
    value: String,
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
    number_value_type: String,
    value: NumberOrSpecialNumber,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NumberOrSpecialNumber {
    Number(f64),
    SpecialNumber(SpecialNumber),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
    #[serde(rename = "type")]
    boolean_value_type: String,
    value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigIntValue {
    #[serde(rename = "type")]
    bigint_value_type: String,
    value: String,
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
    shared_id: SharedId,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteObjectReference {
    handle: Handle,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    shared_id: Option<SharedId>,
    extensible: Extensible,
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
    symbol_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayRemoteValue {
    #[serde(rename = "type")]
    array_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRemoteValue {
    #[serde(rename = "type")]
    object_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
    #[serde(rename = "type")]
    function_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(flatten)]
    reg_exp_local_value: RegExpLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRemoteValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(flatten)]
    date_local_value: DateLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapRemoteValue {
    #[serde(rename = "type")]
    map_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetRemoteValue {
    #[serde(rename = "type")]
    set_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
    #[serde(rename = "type")]
    weak_map_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
    #[serde(rename = "type")]
    weak_set_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
    #[serde(rename = "type")]
    generator_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
    #[serde(rename = "type")]
    error_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
    #[serde(rename = "type")]
    proxy_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
    #[serde(rename = "type")]
    promise_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
    #[serde(rename = "type")]
    typed_array_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
    #[serde(rename = "type")]
    array_buffer_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListRemoteValue {
    #[serde(rename = "type")]
    node_list_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTMLCollectionRemoteValue {
    #[serde(rename = "type")]
    html_collection_remote_value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRemoteValue {
    #[serde(rename = "type")]
    node_remote_value_type: String,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    shared_id: Option<SharedId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Box<NodeProperties>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeProperties {
    #[serde(rename = "nodeType")]
    node_type: JsUint,
    #[serde(rename = "childNodeCount")]
    child_node_count: JsUint,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<NodeRemoteValue>>,
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    local_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<NodePropertiesMode>,
    #[serde(rename = "namespaceURI", skip_serializing_if = "Option::is_none")]
    namespace_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeValue")]
    node_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shadowRoot")]
    shadow_root: Option<Option<NodeRemoteValue>>,
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
    window_proxy_remote_value_type: String,
    value: WindowProxyProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<Handle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyProperties {
    context: BrowsingContext,
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
    max_dom_depth: Option<JsUint>,
    #[serde(rename = "maxObjectDepth", skip_serializing_if = "Option::is_none")]
    max_object_depth: Option<JsUint>,
    #[serde(rename = "includeShadowTree", skip_serializing_if = "Option::is_none")]
    include_shadow_tree: Option<IncludeShadowTree>,
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
    column_number: JsUint,
    #[serde(rename = "functionName")]
    function_name: String,
    #[serde(rename = "lineNumber")]
    line_number: JsUint,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTrace {
    #[serde(rename = "callFrames")]
    call_frames: Vec<StackFrame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmTarget {
    realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextTarget {
    context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
    ContextTarget(ContextTarget),
    RealmTarget(RealmTarget),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScript {
    method: String,
    params: AddPreloadScriptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptParameters {
    #[serde(rename = "functionDeclaration")]
    function_declaration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Vec<ChannelValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<Vec<BrowsingContext>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disown {
    method: String,
    params: DisownParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisownParameters {
    handles: Vec<Handle>,
    target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunction {
    method: String,
    params: CallFunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunctionParameters {
    #[serde(rename = "functionDeclaration")]
    function_declaration: String,
    #[serde(rename = "awaitPromise")]
    await_promise: bool,
    target: Target,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Vec<LocalValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultOwnership")]
    result_ownership: Option<ResultOwnership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serializationOptions")]
    serialization_options: Option<SerializationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    this: Option<LocalValue>,
    #[serde(rename = "userActivation", skip_serializing_if = "Option::is_none")]
    user_activation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
    method: String,
    params: EvaluateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateParameters {
    expression: String,
    target: Target,
    #[serde(rename = "awaitPromise")]
    await_promise: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resultOwnership")]
    result_ownership: Option<ResultOwnership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serializationOptions")]
    serialization_options: Option<SerializationOptions>,
    #[serde(rename = "userActivation", skip_serializing_if = "Option::is_none")]
    user_activation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealms {
    method: String,
    params: GetRealmsParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<BrowsingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    realm_type: Option<RealmType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScript {
    method: String,
    params: RemovePreloadScriptParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScriptParameters {
    script: PreloadScript,
}
