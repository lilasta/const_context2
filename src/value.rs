use core::marker::{Destruct, PhantomData};
use core::mem::MaybeUninit;
use core::ops::*;

use crate::bytes::Bytes;

pub trait ConstValue: Sized {
    type Type: 'static;
    const VALUE: Self::Type;
}

impl ConstValue for () {
    type Type = ();
    const VALUE: Self::Type = ();
}

pub struct ConstValueInstance<Value: ConstValue>(PhantomData<Value>);

impl<Value: ConstValue> ConstValueInstance<Value> {
    pub const fn new() -> Self {
        const { assert!(core::mem::size_of::<Self>() == 0) };
        Self(PhantomData)
    }

    pub const fn map<Map, R>(self, _: Map) -> ConstValueInstance<ConstMap<Value, Map>>
    where
        Map: ~const Destruct,
        Map: ~const FnOnce(Value::Type) -> R,
        R: 'static,
    {
        ConstValueInstance::new()
    }

    pub const fn inspect<Inspect>(&self, _: Inspect)
    where
        Inspect: ~const Destruct,
        Inspect: ~const FnOnce(Value::Type),
    {
        ConstMap::<Value, Inspect>::VALUE
    }

    pub const fn into_inner(self) -> Value::Type {
        Value::VALUE
    }
}

impl<Value: ConstValue> const Clone for ConstValueInstance<Value> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<Value: ConstValue> const Deref for ConstValueInstance<Value> {
    type Target = Value::Type;

    fn deref(&self) -> &Self::Target {
        const {
            unsafe {
                let bytes = Bytes::new(Value::VALUE);
                bytes.as_ref::<Self::Target>()
            }
        }
    }
}

impl<Value: ConstValue> Copy for ConstValueInstance<Value> {}

impl<Value: ConstValue> const Neg for ConstValueInstance<Value>
where
    Value::Type: ~const Neg,
{
    type Output = ConstValueInstance<ConstNeg<Value>>;

    fn neg(self) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Add<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Add<R::Type>,
{
    type Output = ConstValueInstance<ConstAdd<L, R>>;

    fn add(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Sub<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Sub<R::Type>,
{
    type Output = ConstValueInstance<ConstSub<L, R>>;

    fn sub(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Mul<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Mul<R::Type>,
{
    type Output = ConstValueInstance<ConstMul<L, R>>;

    fn mul(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Div<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Div<R::Type>,
{
    type Output = ConstValueInstance<ConstDiv<L, R>>;

    fn div(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Rem<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Rem<R::Type>,
{
    type Output = ConstValueInstance<ConstRem<L, R>>;

    fn rem(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<Value: ConstValue> const Not for ConstValueInstance<Value>
where
    Value::Type: ~const Not,
{
    type Output = ConstValueInstance<ConstNot<Value>>;

    fn not(self) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const BitAnd<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const BitAnd<R::Type>,
{
    type Output = ConstValueInstance<ConstBitAnd<L, R>>;

    fn bitand(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const BitOr<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const BitOr<R::Type>,
{
    type Output = ConstValueInstance<ConstBitOr<L, R>>;

    fn bitor(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const BitXor<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const BitXor<R::Type>,
{
    type Output = ConstValueInstance<ConstBitXor<L, R>>;

    fn bitxor(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Shl<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Shl<R::Type>,
{
    type Output = ConstValueInstance<ConstShl<L, R>>;

    fn shl(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
    }
}

impl<L: ConstValue, R: ConstValue> const Shr<ConstValueInstance<R>> for ConstValueInstance<L>
where
    L::Type: ~const Shr<R::Type>,
{
    type Output = ConstValueInstance<ConstShr<L, R>>;

    fn shr(self, _: ConstValueInstance<R>) -> Self::Output {
        ConstValueInstance::new()
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
        const { &ConstValueInstance::new() }
    }
}

pub struct ConstLiteral<Type, const BYTES: Bytes>(PhantomData<Type>);

impl<Type: 'static, const BYTES: Bytes> ConstValue for ConstLiteral<Type, BYTES> {
    type Type = Type;
    const VALUE: Self::Type = unsafe { BYTES.as_type::<Self::Type>() };
}

pub struct ConstNeg<Value>(PhantomData<Value>);

impl<Value: ConstValue> ConstValue for ConstNeg<Value>
where
    Value::Type: ~const Neg,
{
    type Type = <Value::Type as Neg>::Output;
    const VALUE: Self::Type = -Value::VALUE;
}

pub struct ConstAdd<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstAdd<L, R>
where
    L::Type: ~const Add<R::Type>,
{
    type Type = <L::Type as Add<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE + R::VALUE;
}

pub struct ConstSub<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstSub<L, R>
where
    L::Type: ~const Sub<R::Type>,
{
    type Type = <L::Type as Sub<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE - R::VALUE;
}

pub struct ConstMul<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstMul<L, R>
where
    L::Type: ~const Mul<R::Type>,
{
    type Type = <L::Type as Mul<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE * R::VALUE;
}

pub struct ConstDiv<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstDiv<L, R>
where
    L::Type: ~const Div<R::Type>,
{
    type Type = <L::Type as Div<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE / R::VALUE;
}

pub struct ConstRem<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstRem<L, R>
where
    L::Type: ~const Rem<R::Type>,
{
    type Type = <L::Type as Rem<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE % R::VALUE;
}

pub struct ConstNot<Value>(PhantomData<Value>);

impl<Value: ConstValue> ConstValue for ConstNot<Value>
where
    Value::Type: ~const Not,
{
    type Type = <Value::Type as Not>::Output;
    const VALUE: Self::Type = !Value::VALUE;
}

pub struct ConstBitAnd<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstBitAnd<L, R>
where
    L::Type: ~const BitAnd<R::Type>,
{
    type Type = <L::Type as BitAnd<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE & R::VALUE;
}

pub struct ConstBitOr<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstBitOr<L, R>
where
    L::Type: ~const BitOr<R::Type>,
{
    type Type = <L::Type as BitOr<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE | R::VALUE;
}

pub struct ConstBitXor<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstBitXor<L, R>
where
    L::Type: ~const BitXor<R::Type>,
{
    type Type = <L::Type as BitXor<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE ^ R::VALUE;
}

pub struct ConstShl<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstShl<L, R>
where
    L::Type: ~const Shl<R::Type>,
{
    type Type = <L::Type as Shl<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE << R::VALUE;
}

pub struct ConstShr<L, R>(PhantomData<(L, R)>);

impl<L: ConstValue, R: ConstValue> ConstValue for ConstShr<L, R>
where
    L::Type: ~const Shr<R::Type>,
{
    type Type = <L::Type as Shr<R::Type>>::Output;
    const VALUE: Self::Type = L::VALUE >> R::VALUE;
}

pub struct ConstIndex<Value, Idx>(PhantomData<(Value, Idx)>);

impl<Value: ConstValue, Idx: ConstValue> ConstValue for ConstIndex<Value, Idx>
where
    Value::Type: ~const Index<Idx::Type> + ~const Destruct,
    <Value::Type as Index<Idx::Type>>::Output: Sized + ~const Clone,
{
    type Type = <Value::Type as Index<Idx::Type>>::Output;
    const VALUE: Self::Type = Value::VALUE[Idx::VALUE].clone();
}

pub struct ConstMap<V, F>(PhantomData<(V, F)>);

impl<V, F, R> ConstValue for ConstMap<V, F>
where
    V: ConstValue,
    F: ~const FnOnce(V::Type) -> R,
    R: 'static,
{
    type Type = R;
    const VALUE: Self::Type = {
        // HACK: Functions must be zero-sized. This rejects closures with captured values.
        assert!(core::mem::size_of::<F>() == 0);

        // SAFETY: The type `F` is ZST, so there is no memory to initialize.
        #[allow(clippy::uninit_assumed_init)]
        let f = unsafe { MaybeUninit::<F>::uninit().assume_init() };

        // call...
        f(V::VALUE)
    };
}

pub macro const_value($ty:ty, $init:expr) {{
    struct __ConstValue;

    impl $crate::value::ConstValue for __ConstValue {
        type Type = $ty;
        const VALUE: Self::Type = $init;
    }

    $crate::value::ConstValueInstance::<__ConstValue>::new()
}}
