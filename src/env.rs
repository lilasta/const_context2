use core::any::TypeId;
use core::intrinsics::const_allocate;
use core::marker::PhantomData;

use crate::bytes::Bytes;
use crate::value::{ConstValue, ConstValueInstance};

pub struct Environment<List: VariableList>(PhantomData<List>);

impl Environment<VariableListEnd> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<List: VariableList> Environment<List> {
    pub const fn get<Var: ConstVariable>(
        &self,
    ) -> ConstValueInstance<ConstVariableFind<List, Var::Key, Var::Value>> {
        ConstValueInstance::new()
    }

    pub const fn set<Var: ConstVariable, Value: ConstValue>(
        self,
        _: ConstValueInstance<Value>,
    ) -> Environment<impl VariableList> {
        const { Environment(PhantomData::<VariableListHas<Var::Key, Value, List>>) }
    }

    pub const fn unset<Var: ConstVariable>(self) -> Environment<impl VariableList> {
        const { Environment(PhantomData::<VariableListRemoved<Var::Key, List>>) }
    }

    pub const fn auto<Next: VariableList>(self) -> Environment<Next> {
        const { Environment(PhantomData) }
    }
}

pub struct VariableListEnd;

pub struct VariableListHas<Key, Value: ConstValue, Next>(PhantomData<(Key, Value, Next)>);

pub struct VariableListRemoved<Key, Next>(PhantomData<(Key, Next)>);

pub enum VariableListValue<T> {
    End,
    Has(T),
    Removed,
}

pub trait VariableList {
    type Next: VariableList;
    type Key: 'static;
    type Value: 'static;
    const BYTES: VariableListValue<Bytes>;
}

impl VariableList for VariableListEnd {
    type Next = VariableListEnd;
    type Key = ();
    type Value = ();
    const BYTES: VariableListValue<Bytes> = VariableListValue::End;
}

impl<Key: 'static, Value: ConstValue, Next: VariableList> VariableList
    for VariableListHas<Key, Value, Next>
{
    type Next = Next;
    type Key = Key;
    type Value = Value::Type;
    const BYTES: VariableListValue<Bytes> = VariableListValue::Has(Value::BYTES);
}

impl<Key: 'static, Next: VariableList> VariableList for VariableListRemoved<Key, Next> {
    type Next = Next;
    type Key = Key;
    type Value = ();
    const BYTES: VariableListValue<Bytes> = VariableListValue::Removed;
}

pub struct ConstVariableFind<List, Key, Value>(PhantomData<(List, Key, Value)>);

impl<List, Key, Value> ConstValue for ConstVariableFind<List, Key, Value>
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    type Type = Value;
    const BYTES: Bytes = find_variable::<List, Key, Value>();
}

pub trait ConstVariable {
    type Key: 'static;
    type Value: 'static;
}

impl ConstVariable for () {
    type Key = ();
    type Value = ();
}

impl<K, V> ConstVariable for (K, V)
where
    K: 'static,
    V: 'static,
{
    type Key = K;
    type Value = V;
}

const fn error_not_found<Key>() -> &'static str {
    let type_name = core::any::type_name::<Key>();
    str_concat(
        str_concat("The key `", type_name),
        "` is not found in current context.",
    )
}

const fn error_unexpected_type<Expected, Value>() -> &'static str {
    let type_name_expect = core::any::type_name::<Expected>();
    let type_name_value = core::any::type_name::<Value>();
    str_concat(
        str_concat("Mismatched types: expected `", type_name_expect),
        str_concat("`, found `", str_concat(type_name_value, "`.")),
    )
}

const fn str_concat(s1: &str, s2: &str) -> &'static str {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let len = s1.len() + s2.len();

    unsafe {
        let ptr = const_allocate(
            core::mem::size_of::<u8>() * len,
            core::mem::align_of::<u8>(),
        );
        core::ptr::copy(s1.as_ptr(), ptr, s1.len());
        core::ptr::copy(s2.as_ptr(), ptr.add(s1.len()), s2.len());
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr.cast(), len))
    }
}

const fn type_eq<A: 'static, B: 'static>() -> bool {
    let a = TypeId::of::<A>();
    let b = TypeId::of::<B>();
    unsafe { core::mem::transmute::<_, u64>(a) == core::mem::transmute::<_, u64>(b) }
}

#[track_caller]
const fn find_variable<List, Key, Value>() -> Bytes
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    match List::BYTES {
        VariableListValue::End => panic!("{}", error_not_found::<Key>()),
        VariableListValue::Removed if type_eq::<Key, List::Key>() => {
            panic!("{}", error_not_found::<Key>())
        }
        VariableListValue::Has(bytes) if type_eq::<Key, List::Key>() => {
            assert!(
                type_eq::<Value, List::Value>(),
                "{}",
                error_unexpected_type::<Value, List::Value>()
            );
            bytes
        }
        _ => find_variable::<List::Next, Key, Value>(),
    }
}
