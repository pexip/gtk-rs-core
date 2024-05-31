// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{IOStream, SocketConnectable, TlsCertificateFlags, TlsConnection};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GTlsClientConnection")]
    pub struct TlsClientConnection(Interface<ffi::GTlsClientConnection, ffi::GTlsClientConnectionInterface>) @requires TlsConnection, IOStream;

    match fn {
        type_ => || ffi::g_tls_client_connection_get_type(),
    }
}

impl TlsClientConnection {
    pub const NONE: Option<&'static TlsClientConnection> = None;

    #[doc(alias = "g_tls_client_connection_new")]
    pub fn new(
        base_io_stream: &impl IsA<IOStream>,
        server_identity: Option<&impl IsA<SocketConnectable>>,
    ) -> Result<TlsClientConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_tls_client_connection_new(
                base_io_stream.as_ref().to_glib_none().0,
                server_identity.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TlsClientConnection>> Sealed for T {}
}

pub trait TlsClientConnectionExt: IsA<TlsClientConnection> + sealed::Sealed + 'static {
    #[doc(alias = "g_tls_client_connection_copy_session_state")]
    fn copy_session_state(&self, source: &impl IsA<TlsClientConnection>) {
        unsafe {
            ffi::g_tls_client_connection_copy_session_state(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_tls_client_connection_get_accepted_cas")]
    #[doc(alias = "get_accepted_cas")]
    fn accepted_cas(&self) -> Vec<glib::ByteArray> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_tls_client_connection_get_accepted_cas(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_client_connection_get_server_identity")]
    #[doc(alias = "get_server_identity")]
    fn server_identity(&self) -> Option<SocketConnectable> {
        unsafe {
            from_glib_none(ffi::g_tls_client_connection_get_server_identity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_72", deprecated = "Since 2.72")]
    #[allow(deprecated)]
    #[doc(alias = "g_tls_client_connection_get_validation_flags")]
    #[doc(alias = "get_validation_flags")]
    fn validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_tls_client_connection_get_validation_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_client_connection_set_server_identity")]
    fn set_server_identity(&self, identity: &impl IsA<SocketConnectable>) {
        unsafe {
            ffi::g_tls_client_connection_set_server_identity(
                self.as_ref().to_glib_none().0,
                identity.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_72", deprecated = "Since 2.72")]
    #[allow(deprecated)]
    #[doc(alias = "g_tls_client_connection_set_validation_flags")]
    fn set_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            ffi::g_tls_client_connection_set_validation_flags(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            );
        }
    }

    #[doc(alias = "accepted-cas")]
    fn connect_accepted_cas_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accepted_cas_trampoline<
            P: IsA<TlsClientConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsClientConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsClientConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accepted-cas\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accepted_cas_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "server-identity")]
    fn connect_server_identity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_server_identity_trampoline<
            P: IsA<TlsClientConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsClientConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsClientConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::server-identity\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_server_identity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v2_72", deprecated = "Since 2.72")]
    #[doc(alias = "validation-flags")]
    fn connect_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_validation_flags_trampoline<
            P: IsA<TlsClientConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsClientConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsClientConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::validation-flags\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_validation_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TlsClientConnection>> TlsClientConnectionExt for O {}
