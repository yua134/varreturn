pub trait Array {}

#[repr(transparent)]
/// This struct is used to represent a variable-length array that is returned from a function.
/// Static lifetime is might be caused compiler error when using the [`var_return_call`](crate::var_return_call) macro.
pub struct VarReturn<'a, T> {
    slice: &'a mut [T]
}
impl<'a, T> VarReturn<'a, T> {
    #[doc(hidden)]
    pub fn __new(slice: &'a mut [T]) -> Self {
        Self { slice }
    }
}
impl<'a, T> core::ops::Deref for VarReturn<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}
impl<'a, T> core::ops::DerefMut for VarReturn<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slice
    }
}
impl<'a, T> Drop for VarReturn<'a, T> {
    fn drop(&mut self) {
        for item in self.slice.iter_mut() {
            unsafe {
                core::ptr::drop_in_place(item)
            }
        }
    }
}

/// This trait is used as an inner marker trait for getting the associated array type from a type.
/// If you want to implement this trait for your own type, use the [`impl_associated_array`](crate::impl_associated_array) macro.
pub trait AssociatedArray {
    type Array: Array;
}

#[macro_export]
/// This macro is used to call a function that is marked with the [`varreturn_callee`](crate::varreturn_callee) attribute macro.
/// 
/// |$var| - This is the variable that will be bound to [`VarReturn`] returned from the function.
/// $body - This is the body of the macro that will be executed after the function call.
/// [$ty; $count] - This is the type and count of the array that will be returned from the function. The count is not necessary to be a constant expression.
///
/// The macro calculates the buffer size as `size_of::<$ty>() * $count + (align_of::<$ty>() - 1)`.
/// 
/// # Panics
/// This macro will panic if the buffer size exceeds `isize::MAX`. Only 16-bit architectures can trigger panic when the buffer size is overflowed.
/// 
/// # Safety
/// 
/// This macro uses [alloca::with_alloca](https://docs.rs/alloca/latest/alloca/fn.with_alloca.html) to allocate a buffer on the stack. The buffer size is determined by the type and count of the array.
/// According to alloca, the following are triggering undefined behavior (UB) or segfault:
/// 
/// - Using memory allocated by the macro outside of it e.g $body scope is already ended but you somehow managed to store pointer to memory and use it.
/// - Allocating more memory than thread stack size.
macro_rules! var_return_call {
    (
        $generator:ident($($args:expr),*) -> [$ty:ty; $count:expr];
        |$var:ident| $body:expr
    ) => {
        {
            let count = $count;
            let align = ::core::mem::align_of::<$ty>();
            let size = ::core::mem::size_of::<$ty>();
            let bytes = (size).saturating_mul(count);
            assert!(bytes <= ::core::isize::MAX as usize, "Buffer size exceeds isize::MAX");
            // Alignment is always less than 2^29, so the addition will not overflow on 32-bit and 64-bit architectures. On 16-bit architectures, the addition may overflow, so we use checked_add to prevent that.
            // See: https://doc.rust-lang.org/stable/reference/type-layout.html?utm_source=chatgpt.com#r-layout.repr.alignment.constraint-alignment
            let size = if cfg!(target_pointer_width = "16") {
                bytes.checked_add(align - 1).expect("Buffer size overflow")
            } else {
                bytes + (align - 1)
            };
            $crate::__private::with_alloca(size, |buf| {
                let ptr = buf.as_mut_ptr();
                // It never returns usize::MAX because the buffer elements are aligned to 1.
                let offset = ptr.align_offset(align);
                let aligned_ptr = unsafe { ptr.add(offset).cast::<::core::mem::MaybeUninit<$ty>>() };
                let slice = unsafe { ::core::slice::from_raw_parts_mut(aligned_ptr, count) };
                let arr = $crate::__private::__Array::__new(slice, buf);
                let $var = $generator($($args,)* arr);
                $body
            })
        }
    };
}
