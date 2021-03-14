// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DBusInterface;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DBusObject(Interface<ffi::GDBusObject, ffi::GDBusObjectIface>);

    match fn {
        get_type => || ffi::g_dbus_object_get_type(),
    }
}

pub const NONE_DBUS_OBJECT: Option<&DBusObject> = None;

pub trait DBusObjectExt: 'static {
    #[doc(alias = "g_dbus_object_get_interface")]
    fn get_interface(&self, interface_name: &str) -> Option<DBusInterface>;

    #[doc(alias = "g_dbus_object_get_interfaces")]
    fn get_interfaces(&self) -> Vec<DBusInterface>;

    #[doc(alias = "g_dbus_object_get_object_path")]
    fn get_object_path(&self) -> Option<glib::GString>;

    fn connect_interface_added<F: Fn(&Self, &DBusInterface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_interface_removed<F: Fn(&Self, &DBusInterface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DBusObject>> DBusObjectExt for O {
    fn get_interface(&self, interface_name: &str) -> Option<DBusInterface> {
        unsafe {
            from_glib_full(ffi::g_dbus_object_get_interface(
                self.as_ref().to_glib_none().0,
                interface_name.to_glib_none().0,
            ))
        }
    }

    fn get_interfaces(&self) -> Vec<DBusInterface> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_dbus_object_get_interfaces(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_object_path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_dbus_object_get_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_interface_added<F: Fn(&Self, &DBusInterface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn interface_added_trampoline<P, F: Fn(&P, &DBusInterface) + 'static>(
            this: *mut ffi::GDBusObject,
            interface: *mut ffi::GDBusInterface,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DBusObject>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DBusObject::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(interface),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"interface-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    interface_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_interface_removed<F: Fn(&Self, &DBusInterface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn interface_removed_trampoline<P, F: Fn(&P, &DBusInterface) + 'static>(
            this: *mut ffi::GDBusObject,
            interface: *mut ffi::GDBusInterface,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DBusObject>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DBusObject::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(interface),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"interface-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    interface_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DBusObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DBusObject")
    }
}