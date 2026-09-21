use {
    crate::{CrankstartAPI, System, alloc::rc::Rc, util::singleton::Singleton},
    core::{
        any::Any,
        borrow::Borrow,
        cell::{Ref, RefCell, RefMut},
        cmp::{Eq, PartialEq},
        hash::{Hash, Hasher},
        mem::size_of,
        ptr::NonNull,
    },
    fnv::FnvBuildHasher,
    hashbrown::HashSet,
};

pub(crate) trait PtrTrait
where
    Self: From<UntypedPtr> + 'static,
{
    type PDType;
    type State;

    fn new(pd_ptr: NonNull<Self::PDType>, state: Self::State) -> Self {
        let ptr: Self = Self::from(UntypedPtr::new::<Self>(pd_ptr, state));

        assert!(size_of::<Self>() == size_of::<UntypedPtr>());
        assert!(
            (&ptr) as *const Self as *const u8
                == ptr.get_untyped_ptr() as *const UntypedPtr as *const u8
        );

        ptr
    }

    fn get_untyped_ptr(&self) -> &UntypedPtr;

    fn get_untyped_pd_ptr(&self) -> &NonNull<u8> {
        self.get_untyped_ptr().0.get_untyped_pd_ptr()
    }

    fn get_pd_ptr(&self) -> NonNull<Self::PDType> {
        self.get_untyped_pd_ptr().cast()
    }

    fn get_ptr_inner(&self) -> &PtrInner<Self> {
        // `self` should have been constructed via `new` above.
        self.get_untyped_ptr().try_get_ptr_inner().unwrap()
    }

    fn try_borrow_state<'s>(&'s self) -> Option<Ref<'s, Self::State>> {
        self.get_untyped_ptr().try_borrow_state::<Self>()
    }

    fn try_borrow_state_mut<'s>(&'s self) -> Option<RefMut<'s, Self::State>> {
        self.get_untyped_ptr().try_borrow_state_mut::<Self>()
    }

    fn remove_pd_ptr(pd_ptr: NonNull<Self::PDType>, state: &Self::State);
}

pub trait PtrInnerTrait
where
    Self: Any,
{
    fn get_untyped_pd_ptr(&self) -> &NonNull<u8>;

    fn remove_pd_ptr(&self);
}

pub(crate) struct PtrInner<P: PtrTrait> {
    pd_ptr: NonNull<u8>,
    state: RefCell<P::State>,
}

impl<P: PtrTrait> PtrInner<P> {
    fn get_pd_ptr(&self) -> NonNull<P::PDType> {
        self.pd_ptr.cast()
    }

    fn try_borrow_state<'s>(&'s self) -> Option<Ref<'s, P::State>> {
        self.state.try_borrow().ok()
    }

    fn try_borrow_state_mut<'s>(&'s self) -> Option<RefMut<'s, P::State>> {
        self.state.try_borrow_mut().ok()
    }
}

impl<P: PtrTrait> PtrInnerTrait for PtrInner<P> {
    fn get_untyped_pd_ptr(&self) -> &NonNull<u8> {
        &self.pd_ptr
    }

    fn remove_pd_ptr(&self) {
        P::remove_pd_ptr(self.get_pd_ptr(), &*self.state.borrow());
    }
}

#[derive(Clone)]
pub(crate) struct UntypedPtr(Rc<dyn PtrInnerTrait>);

impl UntypedPtr {
    pub(crate) fn new<P: PtrTrait>(pd_ptr: NonNull<P::PDType>, state: P::State) -> Self {
        let pd_ptr: NonNull<u8> = pd_ptr.cast();
        let state: RefCell<P::State> = RefCell::new(state);
        let ptr: Self = Self(Rc::new(PtrInner::<P> { pd_ptr, state }));

        PtrManager::handle_new_ptr(&ptr);

        ptr
    }

    fn try_get_ptr_inner<P: PtrTrait>(&self) -> Option<&PtrInner<P>> {
        ((&*self.0) as &dyn Any).downcast_ref()
    }

    fn try_borrow_state<'s, P: PtrTrait>(&'s self) -> Option<Ref<'s, P::State>> {
        self.try_get_ptr_inner::<P>()
            .and_then(PtrInner::try_borrow_state)
    }

    fn try_borrow_state_mut<'s, P: PtrTrait>(&'s self) -> Option<RefMut<'s, P::State>> {
        self.try_get_ptr_inner::<P>()
            .and_then(PtrInner::try_borrow_state_mut)
    }
}

impl Borrow<NonNull<u8>> for UntypedPtr {
    fn borrow(&self) -> &NonNull<u8> {
        self.0.get_untyped_pd_ptr()
    }
}

impl Drop for UntypedPtr {
    fn drop(&mut self) {
        PtrManager::handle_dropped_ptr(self);
    }
}

impl Eq for UntypedPtr {}

impl Hash for UntypedPtr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        <UntypedPtr as Borrow<NonNull<u8>>>::borrow(self).hash(state);
    }
}

impl PartialEq for UntypedPtr {
    fn eq(&self, other: &Self) -> bool {
        <UntypedPtr as Borrow<NonNull<u8>>>::borrow(self)
            == <UntypedPtr as Borrow<NonNull<u8>>>::borrow(other)
    }
}

#[derive(Default)]
pub(crate) struct PtrManager(HashSet<UntypedPtr, FnvBuildHasher>);

impl PtrManager {
    pub fn try_get_ptr<P: PtrTrait>(&self, ptr_inner: &PtrInner<P>) -> Option<P> {
        self.0.get(&ptr_inner.pd_ptr).cloned().map(P::from)
    }

    fn is_tracked(&self, ptr: &UntypedPtr) -> bool {
        self.0.contains(ptr)
    }

    fn handle_new_ptr(ptr: &UntypedPtr) {
        let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

        let mut ptr_manager: RefMut<Self> = crankstart_api.ptr_manager.borrow_mut();

        // Calling `0.insert` for a key that's already present wouldn't put things in a bad state,
        // but cloning does unnecessary bookkeeping in that case.
        if !ptr_manager.is_tracked(ptr) {
            ptr_manager.0.insert(ptr.clone());
        }
    }

    fn handle_dropped_ptr(ptr: &UntypedPtr) {
        // If there are two strong pointers to the same object...
        (Rc::strong_count(&ptr.0) == 2_usize).then(|| {
            let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

            let mut ptr_manager: RefMut<Self> = crankstart_api.ptr_manager.borrow_mut();

            // and one of which is a copy in the tracked set (ptr could be that copy at this point),
            // then one copy is in the lieve set and the other is one of the user's. The API
            // restricts a pointer from being removed from the tracked set before the last user copy
            // drops.
            ptr_manager.is_tracked(ptr).then(|| ptr_manager.0.take(ptr))
        });

        if Rc::strong_count(&ptr.0) == 1_usize {
            ptr.0.remove_pd_ptr();
        }
    }
}

impl System for PtrManager {
    fn update() {}
}
