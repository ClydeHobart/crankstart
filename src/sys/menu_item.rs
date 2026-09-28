use {
    crate::{
        CrankstartAPI, define_enum_with_count, ensure,
        pd_api::PDMenuItem,
        util::{
            callback::Callback,
            enum_with_count::EnumWithCount,
            ptr::{PtrTrait, UntypedPtr},
            singleton::Singleton,
        },
    },
    anyhow::Result,
    core::{cell::Ref, ptr::NonNull},
    static_assertions::const_assert_eq,
};

pub const MAX_OPTION_COUNT: usize = 32_usize;

/// Your game can add up to three menu items to the system menu.
pub const MAX_MENU_ITEM_COUNT: usize = 3_usize;

pub type DefaultMenuItemCallback = Callback;
pub type CheckboxMenuItemCallback = Callback<bool>;
pub type OptionsMenuItemCallback = Callback<usize>;

enum StatefulMenuItemKind {
    Default {
        was_removed: bool,
        callback: DefaultMenuItemCallback,
    },
    Checkmark {
        was_removed: bool,
        callback: CheckboxMenuItemCallback,
    },
    Options {
        was_removed: bool,
        option_count: u8,
        callback: OptionsMenuItemCallback,
    },
}

impl StatefulMenuItemKind {
    fn get_stateless(&self) -> MenuItemKind {
        match self {
            StatefulMenuItemKind::Default { .. } => MenuItemKind::Default,
            StatefulMenuItemKind::Checkmark { .. } => MenuItemKind::Checkmark,
            StatefulMenuItemKind::Options { .. } => MenuItemKind::Options,
        }
    }
}

define_enum_with_count! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum MenuItemKind {
        Default,
        Checkmark,
        Options
    }
}

pub struct MenuItemState(StatefulMenuItemKind);

impl MenuItemState {
    pub fn get_kind(&self) -> MenuItemKind {
        self.0.get_stateless()
    }

    pub fn try_get_option_count(&self) -> Option<usize> {
        const_assert_eq!(MenuItemKind::COUNT, 3_usize);

        match self.0 {
            StatefulMenuItemKind::Options { option_count, .. } => Some(option_count as usize),
            _ => None,
        }
    }

    pub(super) fn new_default(callback: DefaultMenuItemCallback) -> Self {
        Self(StatefulMenuItemKind::Default {
            was_removed: false,
            callback,
        })
    }

    pub(super) fn new_checkmark(callback: CheckboxMenuItemCallback) -> Self {
        Self(StatefulMenuItemKind::Checkmark {
            was_removed: false,
            callback,
        })
    }

    pub(super) fn try_new_options(
        option_count: usize,
        callback: OptionsMenuItemCallback,
    ) -> Result<Self> {
        ensure!(option_count < MAX_OPTION_COUNT);

        Ok(Self(StatefulMenuItemKind::Options {
            was_removed: false,
            option_count: option_count as u8,
            callback,
        }))
    }

    pub(super) fn invoke_callback(&self, menu_item: &MenuItemPtr) {
        let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

        match &self.0 {
            StatefulMenuItemKind::Default { callback, .. } => {
                callback.invoke(());
            }
            StatefulMenuItemKind::Checkmark { callback, .. } => {
                callback.invoke(
                    crankstart_api
                        .system
                        .get_menu_item_value(menu_item)
                        .unwrap()
                        != 0_usize,
                );
            }
            StatefulMenuItemKind::Options { callback, .. } => callback.invoke(
                crankstart_api
                    .system
                    .get_menu_item_value(menu_item)
                    .unwrap(),
            ),
        }
    }

    pub(super) fn mark_as_removed(&mut self) {
        *match &mut self.0 {
            StatefulMenuItemKind::Default { was_removed, .. } => was_removed,
            StatefulMenuItemKind::Checkmark { was_removed, .. } => was_removed,
            StatefulMenuItemKind::Options { was_removed, .. } => was_removed,
        } = true;
    }

    fn was_removed(&self) -> bool {
        *match &self.0 {
            StatefulMenuItemKind::Default { was_removed, .. } => was_removed,
            StatefulMenuItemKind::Checkmark { was_removed, .. } => was_removed,
            StatefulMenuItemKind::Options { was_removed, .. } => was_removed,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MenuItemPtr(UntypedPtr);

impl From<UntypedPtr> for MenuItemPtr {
    fn from(value: UntypedPtr) -> Self {
        Self(value)
    }
}

impl PtrTrait for MenuItemPtr {
    type PDType = PDMenuItem;

    type State = MenuItemState;

    fn get_untyped_ptr(&self) -> &UntypedPtr {
        &self.0
    }

    fn remove_pd_ptr(pd_ptr: NonNull<Self::PDType>, state: &Self::State) {
        if !state.was_removed() {
            CrankstartAPI::get()
                .system
                .remove_menu_item_internal(pd_ptr);
        }
    }
}
