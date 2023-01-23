use core::any::TypeId;
use core::intrinsics::const_allocate;
use core::marker::PhantomData;

use crate::value::{ConstValue, ConstValueInstance};

pub struct Environment<List: VariableList>(PhantomData<List>);

impl Environment<VariableListEnd> {
    pub const fn new() -> Self {
        const { assert!(core::mem::size_of::<Self>() == 0) };
        Self(PhantomData)
    }
}

impl<List: VariableList> Environment<List> {
    pub const fn get<Var: ConstVariable>(
        &self,
    ) -> ConstValueInstance<FindConstVariable<List, Var::Key, Var::Value>> {
        ConstValueInstance::new()
    }

    pub const fn has<Var: ConstVariable>(
        &self,
    ) -> ConstValueInstance<HasConstVariable<List, Var::Key, Var::Value>> {
        ConstValueInstance::new()
    }

    pub const fn set<Var: ConstVariable>(
        self,
        value: ConstValueInstance<impl ConstValue>,
    ) -> Environment<impl VariableList> {
        #[inline(always)]
        const fn set_concrete<Key, Value, List>(
            _: ConstValueInstance<Value>,
        ) -> Environment<VariableListHas<Key, Value, List>>
        where
            Key: 'static,
            Value: ConstValue,
            List: VariableList,
        {
            Environment(PhantomData)
        }

        set_concrete::<Var::Key, _, List>(value)
    }

    pub const fn unset<Var: ConstVariable>(
        self,
    ) -> Environment<VariableListRemoved<Var::Key, List>> {
        self.auto()
    }

    pub const fn auto<Next: VariableList>(self) -> Environment<Next> {
        Environment(PhantomData)
    }
}

pub struct VariableListEnd;

pub struct VariableListHas<Key, Value: ConstValue, Next>(PhantomData<(Key, Value, Next)>);

pub struct VariableListRemoved<Key, Next>(PhantomData<(Key, Next)>);

pub enum VariableListKind {
    End,
    Has,
    Removed,
}

pub trait VariableList {
    type Next: VariableList;
    type Key: 'static;
    type Value: ConstValue;
    const KIND: VariableListKind;
}

impl VariableList for VariableListEnd {
    type Next = VariableListEnd;
    type Key = ();
    type Value = ();
    const KIND: VariableListKind = VariableListKind::End;
}

impl<Key: 'static, Value: ConstValue, Next: VariableList> VariableList
    for VariableListHas<Key, Value, Next>
{
    type Next = Next;
    type Key = Key;
    type Value = Value;
    const KIND: VariableListKind = VariableListKind::Has;
}

impl<Key: 'static, Next: VariableList> VariableList for VariableListRemoved<Key, Next> {
    type Next = Next;
    type Key = Key;
    type Value = ();
    const KIND: VariableListKind = VariableListKind::Removed;
}

pub struct FindConstVariable<List, Key, Value>(PhantomData<(List, Key, Value)>);

impl<List, Key, Value> ConstValue for FindConstVariable<List, Key, Value>
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    type Type = Value;
    const VALUE: Self::Type = find_variable::<List, Key, Value>();
}

pub struct HasConstVariable<List, Key, Value>(PhantomData<(List, Key, Value)>);

impl<List, Key, Value> ConstValue for HasConstVariable<List, Key, Value>
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    type Type = bool;
    const VALUE: Self::Type = has_variable::<List, Key, Value>();
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
const fn find_variable<List, Key, Value>() -> Value
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    match List::KIND {
        VariableListKind::End => panic!("{}", error_not_found::<Key>()),
        VariableListKind::Removed if type_eq::<Key, List::Key>() => {
            panic!("{}", error_not_found::<Key>())
        }
        VariableListKind::Has if type_eq::<Key, List::Key>() => {
            assert!(
                type_eq::<Value, <List::Value as ConstValue>::Type>(),
                "{}",
                error_unexpected_type::<Value, <List::Value as ConstValue>::Type>()
            );

            unsafe { core::mem::transmute_copy(&<List::Value as ConstValue>::VALUE) }
        }
        _ => find_variable::<List::Next, Key, Value>(),
    }
}

const fn has_variable<List, Key, Value>() -> bool
where
    List: VariableList,
    Key: 'static,
    Value: 'static,
{
    match List::KIND {
        VariableListKind::End => false,
        VariableListKind::Removed if type_eq::<Key, List::Key>() => false,
        VariableListKind::Has if type_eq::<Key, List::Key>() => {
            type_eq::<Value, <List::Value as ConstValue>::Type>()
        }
        _ => has_variable::<List::Next, Key, Value>(),
    }
}
