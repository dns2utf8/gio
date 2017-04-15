// This file was generated by gir (2d591cc) from gir-files (71d73f0)
// DO NOT EDIT

use Action;
use ffi;
use glib;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct SimpleAction(Object<ffi::GSimpleAction>): Action;

    match fn {
        get_type => || ffi::g_simple_action_get_type(),
    }
}

impl SimpleAction {
    pub fn new(name: &str, parameter_type: Option<&glib::VariantTy>) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new(name.to_glib_none().0, parameter_type.to_glib_none().0))
        }
    }

    pub fn new_stateful(name: &str, parameter_type: Option<&glib::VariantTy>, state: &glib::Variant) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new_stateful(name.to_glib_none().0, parameter_type.to_glib_none().0, state.to_glib_none().0))
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::g_simple_action_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_simple_action_set_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_44")]
    pub fn set_state_hint(&self, state_hint: Option<&glib::Variant>) {
        unsafe {
            ffi::g_simple_action_set_state_hint(self.to_glib_none().0, state_hint.to_glib_none().0);
        }
    }

    pub fn connect_activate<F: Fn(&SimpleAction, &Option<glib::Variant>) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SimpleAction, &Option<glib::Variant>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_change_state<F: Fn(&SimpleAction, &Option<glib::Variant>) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SimpleAction, &Option<glib::Variant>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-state",
                transmute(change_state_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline(this: *mut ffi::GSimpleAction, parameter: *mut glib_ffi::GVariant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SimpleAction, &Option<glib::Variant>) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(parameter))
}

unsafe extern "C" fn change_state_trampoline(this: *mut ffi::GSimpleAction, value: *mut glib_ffi::GVariant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SimpleAction, &Option<glib::Variant>) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(value))
}
