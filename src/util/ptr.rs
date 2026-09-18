use {
    crate::{
        CrankstartAPI,
        alloc::{collections::VecDeque, rc::Rc},
        util::singleton::Singleton,
    },
    anyhow::Result,
    core::{
        any::Any,
        cell::{Ref, RefCell, RefMut},
        cmp::{Eq, PartialEq},
        hash::{Hash, Hasher},
        ptr::NonNull,
    },
    fnv::FnvBuildHasher,
    hashbrown::HashSet,
};

pub(crate) trait PtrStateTrait
where
    Self: 'static,
{
    type PDType;

    fn remove_pd_ptr_static(pd_ptr: NonNull<Self::PDType>);
}

pub(crate) trait PtrInnerTrait
where
    Self: Any,
{
    fn get_pd_ptr(&self) -> NonNull<()>;

    fn remove_pd_ptr(&self);
}

pub(crate) struct Ptr<S: PtrStateTrait> {
    pd_ptr: NonNull<S::PDType>,
    state: RefCell<S>,
}

impl<S: PtrStateTrait> Ptr<S> {
    fn try_borrow<'s>(&'s self) -> Option<Ref<'s, S>> {
        self.state.try_borrow().ok()
    }

    fn try_borrow_mut<'s>(&'s self) -> Option<RefMut<'s, S>> {
        self.state.try_borrow_mut().ok()
    }
}

impl<S: PtrStateTrait> PtrInnerTrait for Ptr<S> {
    fn get_pd_ptr(&self) -> NonNull<()> {
        self.pd_ptr.cast()
    }

    fn remove_pd_ptr(&self) {
        S::remove_pd_ptr_static(self.pd_ptr);
    }
}

pub(crate) struct UntypedPtr(Rc<dyn PtrInnerTrait>);

impl UntypedPtr {
    pub(crate) fn try_new<S: PtrStateTrait>(pd_ptr: NonNull<S::PDType>, state: S) -> Result<Self> {
        let ptr: Self = Self(Rc::new(Ptr {
            pd_ptr,
            state: RefCell::new(state),
        }));

        // let

        // ptr

        todo!()
    }

    pub(crate) fn is_state<S: PtrStateTrait>(&self) -> bool {
        ((&*self.0) as &dyn Any).is::<Ptr<S>>()
    }

    pub(crate) fn try_get<S: PtrStateTrait>(&self) -> Option<&Ptr<S>> {
        ((&*self.0) as &dyn Any).downcast_ref()
    }

    pub(crate) fn try_borrow<'p, S: PtrStateTrait>(&'p self) -> Option<Ref<'p, S>> {
        self.try_get::<S>().and_then(Ptr::try_borrow)
    }

    pub(crate) fn try_borrow_mut<'p, S: PtrStateTrait>(&'p mut self) -> Option<RefMut<'p, S>> {
        self.try_get::<S>().and_then(Ptr::try_borrow_mut)
    }
}

impl Drop for UntypedPtr {
    fn drop(&mut self) {
        if Rc::strong_count(&self.0) == 1_usize {
            self.0.remove_pd_ptr();
        }
    }
}

impl Eq for UntypedPtr {}

impl Hash for UntypedPtr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get_pd_ptr().hash(state);
    }
}

impl PartialEq for UntypedPtr {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_pd_ptr() == other.0.get_pd_ptr()
    }
}

type UntypedPtrSet = HashSet<UntypedPtr, FnvBuildHasher>;

#[derive(Default)]
pub(crate) struct PtrManager {
    live_ptrs: UntypedPtrSet,
    ptrs_to_remove: UntypedPtrSet,
}

impl PtrManager {
    pub(crate) fn try_remove_ptr(&mut self, ptr: UntypedPtr) -> Result<(), usize> {
        // self.0.contains(&ptr)
        todo!()
    }
}
