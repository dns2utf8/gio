// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Permission(Object<ffi::GPermission, ffi::GPermissionClass, PermissionClass>);

    match fn {
        get_type => || ffi::g_permission_get_type(),
    }
}

pub const NONE_PERMISSION: Option<&Permission> = None;

pub trait PermissionExt: 'static {
    fn acquire<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn acquire_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn acquire_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    fn get_allowed(&self) -> bool;

    fn get_can_acquire(&self) -> bool;

    fn get_can_release(&self) -> bool;

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool);

    fn release<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn release_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn release_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    fn connect_property_allowed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_acquire_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_release_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Permission>> PermissionExt for O {
    fn acquire<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_acquire(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn acquire_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn acquire_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_acquire_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = acquire_async_trampoline::<Q>;
        unsafe {
            ffi::g_permission_acquire_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn acquire_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.acquire_async(
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn get_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_allowed(self.as_ref().to_glib_none().0))
        }
    }

    fn get_can_acquire(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_acquire(self.as_ref().to_glib_none().0))
        }
    }

    fn get_can_release(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_release(self.as_ref().to_glib_none().0))
        }
    }

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe {
            ffi::g_permission_impl_update(self.as_ref().to_glib_none().0, allowed.to_glib(), can_acquire.to_glib(), can_release.to_glib());
        }
    }

    fn release<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_release(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn release_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn release_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_release_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = release_async_trampoline::<Q>;
        unsafe {
            ffi::g_permission_release_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn release_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.release_async(
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn connect_property_allowed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::allowed\0".as_ptr() as *const _,
                Some(transmute(notify_allowed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_can_acquire_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::can-acquire\0".as_ptr() as *const _,
                Some(transmute(notify_can_acquire_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_can_release_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::can-release\0".as_ptr() as *const _,
                Some(transmute(notify_can_release_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_allowed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GPermission, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Permission> {
    let f: &F = &*(f as *const F);
    f(&Permission::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_can_acquire_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GPermission, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Permission> {
    let f: &F = &*(f as *const F);
    f(&Permission::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_can_release_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GPermission, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Permission> {
    let f: &F = &*(f as *const F);
    f(&Permission::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Permission")
    }
}
