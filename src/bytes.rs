use core::intrinsics::const_allocate;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bytes(&'static [u8]);

impl Bytes {
    pub const fn new<T>(value: T) -> Self {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();

        let bytes = unsafe {
            let ptr = const_allocate(size, align);
            core::ptr::write(ptr.cast(), value);
            core::slice::from_raw_parts(ptr.cast(), size)
        };

        Self(bytes)
    }

    pub const unsafe fn as_type<T>(self) -> T {
        let Self(bytes) = self;
        core::ptr::read(bytes.as_ptr().cast())
    }

    pub const unsafe fn as_ref<T>(self) -> &'static T {
        let Self(bytes) = self;
        &*bytes.as_ptr().cast()
    }
}
