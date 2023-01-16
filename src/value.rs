use core::marker::{Destruct, PhantomData};
use core::mem::MaybeUninit;
use core::ops::*;

use crate::bytes::Bytes;

pub trait ConstValue: Sized {
    type Type: 'static;
    const BYTES: Bytes;
}

pub struct ConstValueInstance<Value: ConstValue>(PhantomData<Value>);

impl<Value: ConstValue> ConstValueInstance<Value> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    pub const fn map<Map, R>(self, _: Map) -> ConstValueInstance<ConstMap<Value, Map>>
    where
        Map: ~const Destruct,
        Map: ~const FnOnce(Value::Type) -> R,
        R: 'static,
    {
        ConstValueInstance(PhantomData)
    }

    pub const fn inspect<Inspect>(&self, _: Inspect)
    where
        Inspect: ~const Destruct,
        Inspect: ~const FnOnce(Value::Type),
    {
        const { ConstValueInstance(PhantomData::<ConstMap<Value, Inspect>>).unwrap() }
    }

    pub const fn unwrap(self) -> Value::Type {
        const { Value::BYTES.with_type::<Value::Type>() }
    }
}

impl<Value: ConstValue> const Neg for ConstValueInstance<Value>
where
    Value::Type: ~const Neg,
{
    type Output = ConstValueInstance<ConstNeg<Value>>;

    fn neg(self) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Add<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Add<R::Type>,
{
    type Output = ConstValueInstance<ConstAdd<L, R>>;

    fn add(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Sub<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Sub<R::Type>,
{
    type Output = ConstValueInstance<ConstSub<L, R>>;

    fn sub(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Mul<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Mul<R::Type>,
{
    type Output = ConstValueInstance<ConstMul<L, R>>;

    fn mul(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Div<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Div<R::Type>,
{
    type Output = ConstValueInstance<ConstDiv<L, R>>;

    fn div(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Rem<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Rem<R::Type>,
{
    type Output = ConstValueInstance<ConstRem<L, R>>;

    fn rem(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<Value: ConstValue> const Not for ConstValueInstance<Value>
where
    Value::Type: ~const Not,
{
    type Output = ConstValueInstance<ConstNot<Value>>;

    fn not(self) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const BitAnd<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const BitAnd<R::Type>,
{
    type Output = ConstValueInstance<ConstBitAnd<L, R>>;

    fn bitand(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const BitOr<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const BitOr<R::Type>,
{
    type Output = ConstValueInstance<ConstBitOr<L, R>>;

    fn bitor(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const BitXor<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const BitXor<R::Type>,
{
    type Output = ConstValueInstance<ConstBitXor<L, R>>;

    fn bitxor(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Shl<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Shl<R::Type>,
{
    type Output = ConstValueInstance<ConstShl<L, R>>;

    fn shl(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<L: ConstValue, R: ConstValue> const Shr<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Shr<R::Type>,
{
    type Output = ConstValueInstance<ConstShr<L, R>>;

    fn shr(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance(PhantomData)
    }
}

impl<Value: ConstValue, Idx: ConstValue> const Index<ConstValueInstance<Idx>>
    for ConstValueInstance<Value>
where
    Value::Type: ~const Index<Idx::Type> + ~const Destruct,
    <Value::Type as Index<Idx::Type>>::Output: Sized + ~const Clone,
{
    type Output = ConstValueInstance<ConstIndex<Value, Idx>>;

    fn index(&self, _: ConstValueInstance<Idx>) -> &Self::Output {
        const { &ConstValueInstance(PhantomData) }
    }
}

pub struct ConstLiteral<Type, const BYTES: Bytes>(PhantomData<Type>);

impl<Type: 'static, const BYTES: Bytes> ConstValue for ConstLiteral<Type, BYTES> {
    type Type = Type;
    const BYTES: Bytes = BYTES;
}

pub struct ConstNeg<Value>(PhantomData<Value>);

impl<Value: ConstValue> ConstValue for ConstNeg<Value>
where
    Value::Type: ~const Neg,
{
    type Type = <Value::Type as Neg>::Output;
    const BYTES: Bytes = Bytes::new(-Value::BYTES.with_type::<Value::Type>());
}

pub struct ConstAdd<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstAdd<L, R>
where
    L::Type: ~const Add<R::Type>,
{
    type Type = <L::Type as Add<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() + R::BYTES.with_type::<R::Type>());
}

pub struct ConstSub<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstSub<L, R>
where
    L::Type: ~const Sub<R::Type>,
{
    type Type = <L::Type as Sub<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() - R::BYTES.with_type::<R::Type>());
}

pub struct ConstMul<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstMul<L, R>
where
    L::Type: ~const Mul<R::Type>,
{
    type Type = <L::Type as Mul<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() * R::BYTES.with_type::<R::Type>());
}

pub struct ConstDiv<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstDiv<L, R>
where
    L::Type: ~const Div<R::Type>,
{
    type Type = <L::Type as Div<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() / R::BYTES.with_type::<R::Type>());
}

pub struct ConstRem<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstRem<L, R>
where
    L::Type: ~const Rem<R::Type>,
{
    type Type = <L::Type as Rem<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() % R::BYTES.with_type::<R::Type>());
}

pub struct ConstNot<Value>(PhantomData<Value>);

impl<Value: ConstValue> ConstValue for ConstNot<Value>
where
    Value::Type: ~const Not,
{
    type Type = <Value::Type as Not>::Output;
    const BYTES: Bytes = Bytes::new(!Value::BYTES.with_type::<Value::Type>());
}

pub struct ConstBitAnd<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstBitAnd<L, R>
where
    L::Type: ~const BitAnd<R::Type>,
{
    type Type = <L::Type as BitAnd<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() & R::BYTES.with_type::<R::Type>());
}

pub struct ConstBitOr<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstBitOr<L, R>
where
    L::Type: ~const BitOr<R::Type>,
{
    type Type = <L::Type as BitOr<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() | R::BYTES.with_type::<R::Type>());
}

pub struct ConstBitXor<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstBitXor<L, R>
where
    L::Type: ~const BitXor<R::Type>,
{
    type Type = <L::Type as BitXor<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() ^ R::BYTES.with_type::<R::Type>());
}

pub struct ConstShl<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstShl<L, R>
where
    L::Type: ~const Shl<R::Type>,
{
    type Type = <L::Type as Shl<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() << R::BYTES.with_type::<R::Type>());
}

pub struct ConstShr<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstShr<L, R>
where
    L::Type: ~const Shr<R::Type>,
{
    type Type = <L::Type as Shr<R::Type>>::Output;
    const BYTES: Bytes =
        Bytes::new(L::BYTES.with_type::<L::Type>() >> R::BYTES.with_type::<R::Type>());
}

pub struct ConstDeref<Value>(PhantomData<Value>);

impl<Value: ConstValue> ConstValue for ConstDeref<Value>
where
    Value::Type: ~const Deref + ~const Destruct,
    <Value::Type as Deref>::Target: Sized,
{
    type Type = <Value::Type as Deref>::Target;
    const BYTES: Bytes = Bytes::new(Value::BYTES.with_type::<Value::Type>().deref());
}

pub struct ConstDerefMut<Value>(PhantomData<Value>);

impl<Value: ConstValue> ConstValue for ConstDerefMut<Value>
where
    Value::Type: ~const DerefMut + ~const Destruct,
    <Value::Type as Deref>::Target: Sized,
{
    type Type = <Value::Type as Deref>::Target;
    const BYTES: Bytes = Bytes::new(Value::BYTES.with_type::<Value::Type>().deref_mut());
}

pub struct ConstIndex<Value, Idx>(PhantomData<(Value, Idx)>);

impl<Value: ConstValue, Idx: ConstValue> ConstValue for ConstIndex<Value, Idx>
where
    Value::Type: ~const Index<Idx::Type> + ~const Destruct,
    <Value::Type as Index<Idx::Type>>::Output: Sized + ~const Clone,
{
    type Type = <Value::Type as Index<Idx::Type>>::Output;
    const BYTES: Bytes = {
        let arr = Value::BYTES.with_type::<Value::Type>();
        let idx = Idx::BYTES.with_type::<Idx::Type>();
        Bytes::new(arr[idx].clone())
    };
}

pub struct ConstMap<V, F>(PhantomData<(V, F)>);

impl<V, F, R> ConstValue for ConstMap<V, F>
where
    V: ConstValue,
    F: ~const FnOnce(V::Type) -> R,
    R: 'static,
{
    type Type = R;
    const BYTES: Bytes = {
        let f = unsafe { MaybeUninit::<F>::uninit().assume_init() };
        let r = f(V::BYTES.with_type::<V::Type>());
        Bytes::new(r)
    };
}

pub macro const_value($ty:ty, $init:expr) {{
    $crate::value::ConstValueInstance::<
        $crate::value::ConstLiteral<$ty, { $crate::bytes::Bytes::new::<$ty>($init) }>,
    >::new()
}}
