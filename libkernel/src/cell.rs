use core::cell::UnsafeCell;

pub struct SyncRefCell<T> {
    cell: UnsafeCell<Option<T>>,
}

unsafe impl<T> Send for SyncRefCell<T> {}
unsafe impl<T> Sync for SyncRefCell<T> {}
impl<T> SyncRefCell<T> {
    pub const fn empty() -> Self {
        Self {
            cell: UnsafeCell::new(None),
        }
    }

    pub const fn new(obj: T) -> Self {
        Self {
            cell: UnsafeCell::new(Some(obj)),
        }
    }

    pub fn set(&self, obj: T) {
        unsafe { *self.cell.get() = Some(obj) }
    }

    pub fn borrow<'a>(&'a self) -> Option<&'a T> {
        unsafe { self.cell.get().as_ref().unwrap().as_ref() }
    }
}

pub struct SyncOnceCell<T> {
    inner_cell: core::lazy::OnceCell<T>,
}

unsafe impl<T> Send for SyncOnceCell<T> {}
unsafe impl<T> Sync for SyncOnceCell<T> {}
impl<T> SyncOnceCell<T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner_cell: core::lazy::OnceCell::new(),
        }
    }

    #[inline]
    pub fn set(&self, obj: T) -> Result<(), T> {
        self.inner_cell.set(obj)
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        self.inner_cell.get()
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.inner_cell.get_mut()
    }
}