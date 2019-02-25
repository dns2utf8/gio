// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use SocketAddress;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct SocketAddressEnumerator(Object<ffi::GSocketAddressEnumerator, ffi::GSocketAddressEnumeratorClass, SocketAddressEnumeratorClass>);

    match fn {
        get_type => || ffi::g_socket_address_enumerator_get_type(),
    }
}

pub const NONE_SOCKET_ADDRESS_ENUMERATOR: Option<&SocketAddressEnumerator> = None;

pub trait SocketAddressEnumeratorExt: 'static {
    fn next<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<SocketAddress, Error>;

    fn next_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketAddress, Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn next_async_future(&self) -> Box_<futures_core::Future<Item = (Self, SocketAddress), Error = (Self, Error)>> where Self: Sized + Clone;
}

impl<O: IsA<SocketAddressEnumerator>> SocketAddressEnumeratorExt for O {
    fn next<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_address_enumerator_next(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn next_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketAddress, Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn next_async_trampoline<Q: FnOnce(Result<SocketAddress, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_address_enumerator_next_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = next_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_address_enumerator_next_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn next_async_future(&self) -> Box_<futures_core::Future<Item = (Self, SocketAddress), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.next_async(
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
}

impl fmt::Display for SocketAddressEnumerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketAddressEnumerator")
    }
}
