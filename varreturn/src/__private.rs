pub use alloca::with_alloca;

pub struct __Array<'a, T> {
    buf: &'a mut [core::mem::MaybeUninit<T>],
    count: usize,
}
impl<'a, T> __Array<'a, T> {
    #[inline(always)]
    pub const fn __new(buf: &'a mut [core::mem::MaybeUninit<T>], _: &'a [core::mem::MaybeUninit<u8>]) -> Self {
        Self { buf, count: 0 }
    }

    #[inline(always)]
    pub fn __copy(self, src: &mut [core::mem::ManuallyDrop<T>]) -> &'a mut [T] {
        let len = src.len();

        let mut guard = DropGuard {
            src,
            armed: true,
        };

        assert!(len <= self.buf.len(), "Buffer overflow");
        assert!(self.count == 0, "Buffer already initialized");

        // Following code will not panic, so we can safely disarm the guard here.
        guard.armed = false;

        let dst_ptr = self.buf.as_mut_ptr() as *mut T;
        unsafe {
            core::ptr::copy_nonoverlapping(guard.src.as_ptr() as *const T, dst_ptr, len);
        }

        unsafe {
            core::slice::from_raw_parts_mut(dst_ptr, len)
        }
    }

    #[inline(always)]
    pub fn __push(&mut self, item: core::mem::ManuallyDrop<T>) {
        let mut bind = [item];
        let mut guard = DropGuard {
            src: bind.as_mut_slice(),
            armed: true,
        };
        assert!(self.count < self.buf.len(), "Buffer overflow");

        // Following code will not panic, so we can safely disarm the guard here.
        guard.armed = false;

        let dst_ptr = self.buf.as_mut_ptr() as *mut T;
        unsafe {
            core::ptr::copy_nonoverlapping(guard.src.as_ptr() as *const T, dst_ptr.add(self.count), 1);
        }
        self.count += 1;
    }

    #[inline(always)]
    pub fn __init(self) -> &'a mut [T] {
        let mut this = core::mem::ManuallyDrop::new(self);
        let dst_ptr = this.buf.as_mut_ptr() as *mut T;
        unsafe {
            core::slice::from_raw_parts_mut(dst_ptr, this.count)
        }
    }
}
impl<'a, T> Drop for __Array<'a, T> {
    fn drop(&mut self) {
        for item in self.buf.iter_mut().take(self.count) {
            unsafe {
                core::ptr::drop_in_place(item.as_mut_ptr())
            }
        }
    }
}

impl<'a, T> crate::imp::Array for __Array<'a, T> {}
impl<'a, T> crate::imp::AssociatedArray for crate::imp::VarReturn<'a, T> {
    type Array = __Array<'a, T>;
}

struct DropGuard<'a, T> {
    src: &'a mut [core::mem::ManuallyDrop<T>],
    armed: bool,
}
impl <'a, T> Drop for DropGuard<'a, T> {
    fn drop(&mut self) {
        if self.armed {
            for item in self.src.iter_mut() {
                unsafe {
                    core::mem::ManuallyDrop::drop(item)
                }
            }
        }
    }
}