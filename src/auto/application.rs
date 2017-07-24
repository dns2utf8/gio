// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use ActionGroup;
use ActionMap;
use ApplicationFlags;
use Cancellable;
use Error;
use File;
#[cfg(feature = "v2_40")]
use Notification;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Application(Object<ffi::GApplication>): ActionGroup, ActionMap;

    match fn {
        get_type => || ffi::g_application_get_type(),
    }
}

impl Application {
    pub fn new<'a, P: Into<Option<&'a str>>>(application_id: P, flags: ApplicationFlags) -> Application {
        let application_id = application_id.into();
        let application_id = application_id.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_application_new(application_id.0, flags.to_glib()))
        }
    }

    pub fn get_default() -> Option<Application> {
        unsafe {
            from_glib_none(ffi::g_application_get_default())
        }
    }

    pub fn id_is_valid(application_id: &str) -> bool {
        unsafe {
            from_glib(ffi::g_application_id_is_valid(application_id.to_glib_none().0))
        }
    }
}

pub trait ApplicationExt {
    fn activate(&self);

    //#[cfg(feature = "v2_42")]
    //fn add_main_option<'a, P: Into<Option<&'a str>>>(&self, long_name: &str, short_name: /*Unimplemented*/Fundamental: Char, flags: /*Ignored*/glib::OptionFlags, arg: /*Ignored*/glib::OptionArg, description: &str, arg_description: P);

    //#[cfg(feature = "v2_40")]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]);

    //#[cfg(feature = "v2_40")]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup);

    #[cfg(feature = "v2_44")]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    fn get_application_id(&self) -> Option<String>;

    //#[cfg(feature = "v2_34")]
    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection>;

    #[cfg(feature = "v2_34")]
    fn get_dbus_object_path(&self) -> Option<String>;

    fn get_flags(&self) -> ApplicationFlags;

    fn get_inactivity_timeout(&self) -> u32;

    #[cfg(feature = "v2_44")]
    fn get_is_busy(&self) -> bool;

    fn get_is_registered(&self) -> bool;

    fn get_is_remote(&self) -> bool;

    #[cfg(feature = "v2_42")]
    fn get_resource_base_path(&self) -> Option<String>;

    fn hold(&self);

    #[cfg(feature = "v2_38")]
    fn mark_busy(&self);

    fn open(&self, files: &[File], hint: &str);

    fn quit(&self);

    fn register<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn release(&self);

    fn run(&self, argv: &[&str]) -> i32;

    #[cfg(feature = "v2_40")]
    fn send_notification<'a, P: Into<Option<&'a str>>>(&self, id: P, notification: &Notification);

    fn set_action_group<'a, P: IsA<ActionGroup> + 'a, Q: Into<Option<&'a P>>>(&self, action_group: Q);

    fn set_application_id<'a, P: Into<Option<&'a str>>>(&self, application_id: P);

    fn set_default(&self);

    fn set_flags(&self, flags: ApplicationFlags);

    fn set_inactivity_timeout(&self, inactivity_timeout: u32);

    #[cfg(feature = "v2_42")]
    fn set_resource_base_path<'a, P: Into<Option<&'a str>>>(&self, resource_path: P);

    #[cfg(feature = "v2_44")]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    #[cfg(feature = "v2_38")]
    fn unmark_busy(&self);

    #[cfg(feature = "v2_40")]
    fn withdraw_notification(&self, id: &str);

    fn set_property_action_group<P: IsA<ActionGroup> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, action_group: Option<&P>);

    fn get_property_application_id(&self) -> Option<String>;

    fn set_property_application_id(&self, application_id: Option<&str>);

    fn get_property_flags(&self) -> ApplicationFlags;

    fn set_property_flags(&self, flags: ApplicationFlags);

    fn get_property_inactivity_timeout(&self) -> u32;

    fn set_property_inactivity_timeout(&self, inactivity_timeout: u32);

    fn get_property_is_registered(&self) -> bool;

    fn get_property_is_remote(&self) -> bool;

    fn get_property_resource_base_path(&self) -> Option<String>;

    fn set_property_resource_base_path(&self, resource_base_path: Option<&str>);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    //fn connect_command_line<Unsupported or ignored types>(&self, f: F) -> u64;

    //#[cfg(feature = "v2_40")]
    //fn connect_handle_local_options<Unsupported or ignored types>(&self, f: F) -> u64;

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Application> + IsA<glib::object::Object>> ApplicationExt for O {
    fn activate(&self) {
        unsafe {
            ffi::g_application_activate(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_42")]
    //fn add_main_option<'a, P: Into<Option<&'a str>>>(&self, long_name: &str, short_name: /*Unimplemented*/Fundamental: Char, flags: /*Ignored*/glib::OptionFlags, arg: /*Ignored*/glib::OptionArg, description: &str, arg_description: P) {
    //    unsafe { TODO: call ffi::g_application_add_main_option() }
    //}

    //#[cfg(feature = "v2_40")]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]) {
    //    unsafe { TODO: call ffi::g_application_add_main_option_entries() }
    //}

    //#[cfg(feature = "v2_40")]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup) {
    //    unsafe { TODO: call ffi::g_application_add_option_group() }
    //}

    #[cfg(feature = "v2_44")]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            ffi::g_application_bind_busy_property(self.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    fn get_application_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_application_id(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_34")]
    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection> {
    //    unsafe { TODO: call ffi::g_application_get_dbus_connection() }
    //}

    #[cfg(feature = "v2_34")]
    fn get_dbus_object_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_dbus_object_path(self.to_glib_none().0))
        }
    }

    fn get_flags(&self) -> ApplicationFlags {
        unsafe {
            from_glib(ffi::g_application_get_flags(self.to_glib_none().0))
        }
    }

    fn get_inactivity_timeout(&self) -> u32 {
        unsafe {
            ffi::g_application_get_inactivity_timeout(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_44")]
    fn get_is_busy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_busy(self.to_glib_none().0))
        }
    }

    fn get_is_registered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_registered(self.to_glib_none().0))
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_remote(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_42")]
    fn get_resource_base_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_resource_base_path(self.to_glib_none().0))
        }
    }

    fn hold(&self) {
        unsafe {
            ffi::g_application_hold(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_38")]
    fn mark_busy(&self) {
        unsafe {
            ffi::g_application_mark_busy(self.to_glib_none().0);
        }
    }

    fn open(&self, files: &[File], hint: &str) {
        let n_files = files.len() as i32;
        unsafe {
            ffi::g_application_open(self.to_glib_none().0, files.to_glib_none().0, n_files, hint.to_glib_none().0);
        }
    }

    fn quit(&self) {
        unsafe {
            ffi::g_application_quit(self.to_glib_none().0);
        }
    }

    fn register<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_application_register(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn release(&self) {
        unsafe {
            ffi::g_application_release(self.to_glib_none().0);
        }
    }

    fn run(&self, argv: &[&str]) -> i32 {
        let argc = argv.len() as i32;
        unsafe {
            ffi::g_application_run(self.to_glib_none().0, argc, argv.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_40")]
    fn send_notification<'a, P: Into<Option<&'a str>>>(&self, id: P, notification: &Notification) {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            ffi::g_application_send_notification(self.to_glib_none().0, id.0, notification.to_glib_none().0);
        }
    }

    fn set_action_group<'a, P: IsA<ActionGroup> + 'a, Q: Into<Option<&'a P>>>(&self, action_group: Q) {
        let action_group = action_group.into();
        let action_group = action_group.to_glib_none();
        unsafe {
            ffi::g_application_set_action_group(self.to_glib_none().0, action_group.0);
        }
    }

    fn set_application_id<'a, P: Into<Option<&'a str>>>(&self, application_id: P) {
        let application_id = application_id.into();
        let application_id = application_id.to_glib_none();
        unsafe {
            ffi::g_application_set_application_id(self.to_glib_none().0, application_id.0);
        }
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_application_set_default(self.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: ApplicationFlags) {
        unsafe {
            ffi::g_application_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn set_inactivity_timeout(&self, inactivity_timeout: u32) {
        unsafe {
            ffi::g_application_set_inactivity_timeout(self.to_glib_none().0, inactivity_timeout);
        }
    }

    #[cfg(feature = "v2_42")]
    fn set_resource_base_path<'a, P: Into<Option<&'a str>>>(&self, resource_path: P) {
        let resource_path = resource_path.into();
        let resource_path = resource_path.to_glib_none();
        unsafe {
            ffi::g_application_set_resource_base_path(self.to_glib_none().0, resource_path.0);
        }
    }

    #[cfg(feature = "v2_44")]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            ffi::g_application_unbind_busy_property(self.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_38")]
    fn unmark_busy(&self) {
        unsafe {
            ffi::g_application_unmark_busy(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_40")]
    fn withdraw_notification(&self, id: &str) {
        unsafe {
            ffi::g_application_withdraw_notification(self.to_glib_none().0, id.to_glib_none().0);
        }
    }

    fn set_property_action_group<P: IsA<ActionGroup> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, action_group: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "action-group".to_glib_none().0, Value::from(action_group).to_glib_none().0);
        }
    }

    fn get_property_application_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "application-id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_application_id(&self, application_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "application-id".to_glib_none().0, Value::from(application_id).to_glib_none().0);
        }
    }

    fn get_property_flags(&self) -> ApplicationFlags {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "flags".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    fn set_property_flags(&self, flags: ApplicationFlags) {
        let flags = flags.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "flags".to_glib_none().0, Value::from(&flags).to_glib_none().0);
        }
    }

    fn get_property_inactivity_timeout(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "inactivity-timeout".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_inactivity_timeout(&self, inactivity_timeout: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "inactivity-timeout".to_glib_none().0, Value::from(&inactivity_timeout).to_glib_none().0);
        }
    }

    fn get_property_is_registered(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-registered".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_is_remote(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-remote".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_resource_base_path(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "resource-base-path".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_resource_base_path(&self, resource_base_path: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "resource-base-path".to_glib_none().0, Value::from(resource_base_path).to_glib_none().0);
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_command_line<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored command_line: Gio.ApplicationCommandLine
    //}

    //#[cfg(feature = "v2_40")]
    //fn connect_handle_local_options<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored options: GLib.VariantDict
    //}

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "shutdown",
                transmute(shutdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "startup",
                transmute(startup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GApplication, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Application::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn shutdown_trampoline<P>(this: *mut ffi::GApplication, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Application::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn startup_trampoline<P>(this: *mut ffi::GApplication, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Application::from_glib_none(this).downcast_unchecked())
}
