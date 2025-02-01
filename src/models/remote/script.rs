use crate::remote::browsing_context::BrowsingContext;
use crate::remote::browser::UserContext;
use crate::remote::{Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl ChannelValue {
    pub fn new(channel_value_type: String, value: ChannelProperties) -> Self {
        Self {
            channel_value_type,
            value,
        }
    }
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

impl ChannelProperties {
    pub fn new(
        channel: Channel,
        serialization_options: Option<SerializationOptions>,
        ownership: Option<ResultOwnership>,
    ) -> Self {
        Self {
            channel,
            serialization_options,
            ownership,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl EvaluateResultSuccess {
    pub fn new(result: RemoteValue, realm: Realm) -> Self {
        Self {
            evaluate_result_success_type: "success".to_string(),
            result,
            realm,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultException {
    #[serde(rename = "type")]
    pub evaluate_result_exception_type: String,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: ExceptionDetails,
    pub realm: Realm,
}

impl EvaluateResultException {
    pub fn new(exception_details: ExceptionDetails, realm: Realm) -> Self {
        Self {
            evaluate_result_exception_type: "exception".to_string(),
            exception_details,
            realm,
        }
    }
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

impl ExceptionDetails {
    pub fn new(
        column_number: JsUint,
        exception: RemoteValue,
        line_number: JsUint,
        stack_trace: StackTrace,
        text: String,
    ) -> Self {
        Self {
            column_number,
            exception,
            line_number,
            stack_trace,
            text,
        }
    }
}

pub type Handle = String;
pub type InternalId = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl ArrayLocalValue {
    pub fn new(value: ListLocalValue) -> Self {
        Self {
            array_local_value_type: "array".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLocalValue {
    #[serde(rename = "type")]
    pub date_local_value_type: String,
    pub value: String,
}

impl DateLocalValue {
    pub fn new(value: String) -> Self {
        Self {
            date_local_value_type: "date".to_string(),
            value,
        }
    }
}

pub type MappingLocalValue = Vec<(LocalValueOrText, LocalValue)>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl MapLocalValue {
    pub fn new(value: MappingLocalValue) -> Self {
        Self {
            map_local_value_type: "map".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLocalValue {
    #[serde(rename = "type")]
    pub object_local_value_type: String,
    pub value: MappingLocalValue,
}

impl ObjectLocalValue {
    pub fn new(value: MappingLocalValue) -> Self {
        Self {
            object_local_value_type: "object".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpValue {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
}

impl RegExpValue {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        Self { pattern, flags }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpLocalValue {
    #[serde(rename = "type")]
    pub regexp_local_value_type: String,
    pub value: RegExpValue,
}

impl RegExpLocalValue {
    pub fn new(value: RegExpValue) -> Self {
        Self {
            regexp_local_value_type: "regexp".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetLocalValue {
    #[serde(rename = "type")]
    pub set_local_value_type: String,
    pub value: ListLocalValue,
}

impl SetLocalValue {
    pub fn new(value: ListLocalValue) -> Self {
        Self {
            set_local_value_type: "set".to_string(),
            value,
        }
    }
}

pub type PreloadScript = String;
pub type Realm = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl UndefinedValue {
    pub fn new() -> Self {
        Self {
            undefined_value_type: "undefined".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullValue {
    #[serde(rename = "type")]
    pub null_value_type: String,
}

impl NullValue {
    pub fn new() -> Self {
        Self {
            null_value_type: "null".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub string_value_type: String,
    pub value: String,
}

impl StringValue {
    pub fn new(value: String) -> Self {
        Self {
            string_value_type: "string".to_string(),
            value,
        }
    }
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

impl NumberValue {
    pub fn new(value: NumberOrSpecialNumber) -> Self {
        Self {
            number_value_type: "number".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl BooleanValue {
    pub fn new(value: bool) -> Self {
        Self {
            boolean_value_type: "boolean".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigIntValue {
    #[serde(rename = "type")]
    pub bigint_value_type: String,
    pub value: String,
}

impl BigIntValue {
    pub fn new(value: String) -> Self {
        Self {
            bigint_value_type: "bigint".to_string(),
            value,
        }
    }
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
#[serde(untagged)]
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

impl SharedReference {
    pub fn new(shared_id: SharedId, handle: Option<Handle>, extensible: Extensible) -> Self {
        Self {
            shared_id,
            handle,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteObjectReference {
    pub handle: Handle,
    #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
    pub shared_id: Option<SharedId>,
    pub extensible: Extensible,
}

impl RemoteObjectReference {
    pub fn new(handle: Handle, shared_id: Option<SharedId>, extensible: Extensible) -> Self {
        Self {
            handle,
            shared_id,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
#[serde(untagged)]
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

impl SymbolRemoteValue {
    pub fn new(
        symbol_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            symbol_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl ArrayRemoteValue {
    pub fn new(
        array_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<ListRemoteValue>,
    ) -> Self {
        Self {
            array_remote_value_type,
            handle,
            internal_id,
            value,
        }
    }
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

impl ObjectRemoteValue {
    pub fn new(
        object_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<MappingRemoteValue>,
    ) -> Self {
        Self {
            object_remote_value_type,
            handle,
            internal_id,
            value,
        }
    }
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

impl FunctionRemoteValue {
    pub fn new(
        function_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            function_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl RegExpRemoteValue {
    pub fn new(
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        reg_exp_local_value: RegExpLocalValue,
    ) -> Self {
        Self {
            handle,
            internal_id,
            reg_exp_local_value,
        }
    }
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

impl DateRemoteValue {
    pub fn new(
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        date_local_value: DateLocalValue,
    ) -> Self {
        Self {
            handle,
            internal_id,
            date_local_value,
        }
    }
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

impl MapRemoteValue {
    pub fn new(
        map_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<MappingRemoteValue>,
    ) -> Self {
        Self {
            map_remote_value_type,
            handle,
            internal_id,
            value,
        }
    }
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

impl SetRemoteValue {
    pub fn new(
        set_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<ListRemoteValue>,
    ) -> Self {
        Self {
            set_remote_value_type,
            handle,
            internal_id,
            value,
        }
    }
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

impl WeakMapRemoteValue {
    pub fn new(
        weak_map_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            weak_map_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl WeakSetRemoteValue {
    pub fn new(
        weak_set_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            weak_set_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl GeneratorRemoteValue {
    pub fn new(
        generator_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            generator_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl ErrorRemoteValue {
    pub fn new(
        error_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            error_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl ProxyRemoteValue {
    pub fn new(
        proxy_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            proxy_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl PromiseRemoteValue {
    pub fn new(
        promise_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            promise_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl TypedArrayRemoteValue {
    pub fn new(
        typed_array_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            typed_array_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl ArrayBufferRemoteValue {
    pub fn new(
        array_buffer_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            array_buffer_remote_value_type,
            handle,
            internal_id,
        }
    }
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

impl NodeListRemoteValue {
    pub fn new(
        node_list_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<ListRemoteValue>,
    ) -> Self {
        Self {
            node_list_remote_value_type,
            handle,
            internal_id,
            value,
        }
    }
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

impl HTMLCollectionRemoteValue {
    pub fn new(
        html_collection_remote_value_type: String,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<ListRemoteValue>,
    ) -> Self {
        Self {
            html_collection_remote_value_type,
            handle,
            internal_id,
            value,
        }
    }
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

impl NodeRemoteValue {
    pub fn new(
        node_remote_value_type: String,
        shared_id: Option<SharedId>,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
        value: Option<NodeProperties>,
    ) -> Self {
        Self {
            node_remote_value_type,
            shared_id,
            handle,
            internal_id,
            value,
        }
    }
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

impl NodeProperties {
    pub fn new(
        node_type: JsUint,
        child_node_count: JsUint,
        attributes: Option<std::collections::HashMap<String, String>>,
        children: Option<Vec<NodeRemoteValue>>,
        local_name: Option<String>,
        mode: Option<NodePropertiesMode>,
        namespace_uri: Option<String>,
        node_value: Option<String>,
    ) -> Self {
        Self {
            node_type,
            child_node_count,
            attributes,
            children,
            local_name,
            mode,
            namespace_uri,
            node_value,
        }
    }
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

impl WindowProxyRemoteValue {
    pub fn new(
        window_proxy_remote_value_type: String,
        value: WindowProxyProperties,
        handle: Option<Handle>,
        internal_id: Option<InternalId>,
    ) -> Self {
        Self {
            window_proxy_remote_value_type,
            value,
            handle,
            internal_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyProperties {
    pub context: BrowsingContext,
}

impl WindowProxyProperties {
    pub fn new(context: BrowsingContext) -> Self {
        Self { context }
    }
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

impl SerializationOptions {
    pub fn new(
        max_dom_depth: Option<JsUint>,
        max_object_depth: Option<JsUint>,
        include_shadow_tree: Option<IncludeShadowTree>,
    ) -> Self {
        Self {
            max_dom_depth,
            max_object_depth,
            include_shadow_tree,
        }
    }
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

impl StackFrame {
    pub fn new(
        column_number: JsUint,
        function_name: String,
        line_number: JsUint,
        url: String,
    ) -> Self {
        Self {
            column_number,
            function_name,
            line_number,
            url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTrace {
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<StackFrame>,
}

impl StackTrace {
    pub fn new(call_frames: Vec<StackFrame>) -> Self {
        Self { call_frames }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmTarget {
    pub realm: Realm,
}

impl RealmTarget {
    pub fn new(realm: Realm) -> Self {
        Self { realm }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextTarget {
    pub context: BrowsingContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

impl ContextTarget {
    pub fn new(context: BrowsingContext, sandbox: Option<String>) -> Self {
        Self { context, sandbox }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    ContextTarget(ContextTarget),
    RealmTarget(RealmTarget),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScript {
    pub method: String,
    pub params: AddPreloadScriptParameters,
}

impl AddPreloadScript {
    pub fn new(params: AddPreloadScriptParameters) -> Self {
        Self {
            method: "script.addPreloadScript".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPreloadScriptParameters {
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<ChannelValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
}

impl AddPreloadScriptParameters {
    pub fn new(
        function_declaration: String,
        arguments: Option<Vec<ChannelValue>>,
        contexts: Option<Vec<BrowsingContext>>,
        user_contexts: Option<Vec<UserContext>>,
        sandbox: Option<String>,
    ) -> Self {
        Self {
            function_declaration,
            arguments,
            contexts,
            user_contexts,
            sandbox,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disown {
    pub method: String,
    pub params: DisownParameters,
}

impl Disown {
    pub fn new(params: DisownParameters) -> Self {
        Self {
            method: "script.disown".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisownParameters {
    pub handles: Vec<Handle>,
    pub target: Target,
}

impl DisownParameters {
    pub fn new(handles: Vec<Handle>, target: Target) -> Self {
        Self { handles, target }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallFunction {
    pub method: String,
    pub params: CallFunctionParameters,
}

impl CallFunction {
    pub fn new(params: CallFunctionParameters) -> Self {
        Self {
            method: "script.callFunction".to_string(),
            params,
        }
    }
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

impl CallFunctionParameters {
    pub fn new(
        function_declaration: String,
        await_promise: bool,
        target: Target,
        arguments: Option<Vec<LocalValue>>,
        result_ownership: Option<ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        this: Option<LocalValue>,
        user_activation: Option<bool>,
    ) -> Self {
        Self {
            function_declaration,
            await_promise,
            target,
            arguments,
            result_ownership,
            serialization_options,
            this,
            user_activation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
    pub method: String,
    pub params: EvaluateParameters,
}

impl Evaluate {
    pub fn new(params: EvaluateParameters) -> Self {
        Self {
            method: "script.evaluate".to_string(),
            params,
        }
    }
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

impl EvaluateParameters {
    pub fn new(
        expression: String,
        target: Target,
        await_promise: bool,
        result_ownership: Option<ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        user_activation: Option<bool>,
    ) -> Self {
        Self {
            expression,
            target,
            await_promise,
            result_ownership,
            serialization_options,
            user_activation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealms {
    pub method: String,
    pub params: GetRealmsParameters,
}

impl GetRealms {
    pub fn new(params: GetRealmsParameters) -> Self {
        Self {
            method: "script.getRealms".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRealmsParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<BrowsingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub realm_type: Option<RealmType>,
}

impl GetRealmsParameters {
    pub fn new(context: Option<BrowsingContext>, realm_type: Option<RealmType>) -> Self {
        Self {
            context,
            realm_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScript {
    pub method: String,
    pub params: RemovePreloadScriptParameters,
}

impl RemovePreloadScript {
    pub fn new(params: RemovePreloadScriptParameters) -> Self {
        Self {
            method: "script.removePreloadScript".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePreloadScriptParameters {
    pub script: PreloadScript,
}

impl RemovePreloadScriptParameters {
    pub fn new(script: PreloadScript) -> Self {
        Self { script }
    }
}
