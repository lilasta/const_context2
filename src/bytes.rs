use core::intrinsics::const_allocate;

/// Bytes of a value.
///
/// # Motivation
///
/// Const generics can't take a generic type, so we treat all values by reducing them to bytes instead.
/// Also, since the Rust compiler refuses to use interior mutable types in various cases, we avoid annoying errors by using this.
/// (this situation may improve in the future when Rust's std publishes the `Freeze` trait, though.)
///
/// # Safety
///
/// This is meant to be used in const contexts, it should cause SEGV if you use it at runtime.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bytes(&'static [u8]);

impl Bytes {
    /// Creates bytes from a value.
    ///
    /// # Safety
    ///
    /// - This must not be called at runtime, it should cause SEGV.
    /// - The type `T` must not have interior mutability. (must implement `Freeze` trait)
    /// - The type `T` must not be a reference.
    pub(crate) const unsafe fn new<T: Sized>(value: T) -> Self {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();

        let ptr = const_allocate(size, align);
        core::ptr::write(ptr.cast(), value);

        let bytes = core::slice::from_raw_parts(ptr.cast(), size);
        Self(bytes)
    }

    /// Converts bytes into a (typed) value.
    ///
    /// # Safety
    ///
    /// - The type `T` must be the same as the type of the original value.
    /// - The type `T` must not have interior mutability. (must implement `Freeze` trait)
    /// - The type `T` must not be a reference.
    pub const unsafe fn as_type<T>(self) -> T {
        let Self(bytes) = self;

        // This check is weak - the same size doesn't mean the same type.
        assert!(bytes.len() == core::mem::size_of::<T>());

        core::ptr::read(bytes.as_ptr().cast())
    }

    /// Converts bytes into a reference of a (typed) value.
    ///
    /// # Safety
    ///
    /// - The type `T` must be the same as the type of the original value.
    /// - The type `T` must not have interior mutability. (must implement `Freeze` trait)
    /// - The type `T` must not be a reference.
    pub const unsafe fn as_ref<T>(self) -> &'static T {
        let Self(bytes) = self;

        // This check is weak - the same size doesn't mean the same type.
        assert!(bytes.len() == core::mem::size_of::<T>());

        &*bytes.as_ptr().cast()
    }
}
