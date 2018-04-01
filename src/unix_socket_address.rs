// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::path::Path;
use std::ptr;

use glib::object::Downcast;
use glib::translate::*;

use ffi;
use SocketAddress;
use UnixSocketAddress;
use UnixSocketAddressType;

use self::AddressType::*;

pub enum AddressType<'a> {
    Path(&'a Path),
    Anonymous,
    Abstract(&'a [u8]),
    AbstractPadded(&'a [u8]),
}

impl<'a> AddressType<'a> {
    fn to_type(&self) -> UnixSocketAddressType {
        match *self {
            Path(_) => UnixSocketAddressType::Path,
            Anonymous => UnixSocketAddressType::Anonymous,
            Abstract(_) => UnixSocketAddressType::Abstract,
            AbstractPadded(_) => UnixSocketAddressType::AbstractPadded,
        }
    }
}

impl UnixSocketAddress {
    pub fn new_with_type(address_type: AddressType) -> Self {
        let type_ = address_type.to_type();
        let (path, len) =
            match address_type {
                Path(path) => (path.to_glib_none().0, path.as_os_str().len()),
                Abstract(path) | AbstractPadded(path) => (path.to_glib_none().0 as *mut i8, path.len()),
                Anonymous => (ptr::null_mut(), 0),
            };
        unsafe {
            SocketAddress::from_glib_full(ffi::g_unix_socket_address_new_with_type(path, len as i32, type_.to_glib()))
                .downcast_unchecked()
        }
    }
}
