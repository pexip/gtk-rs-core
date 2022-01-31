// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `Value` binding and helper traits.
//!
//! The type of a [`Value`](struct.Value.html) is dynamic in that it generally
//! isn't known at compile time but once created a `Value` can't change its
//! type.
//!
//! [`SendValue`](struct.SendValue.html) is a version of [`Value`](struct.Value.html)
//! that can only store types that implement `Send` and as such implements `Send` itself. It
//! dereferences to `Value` so it can be used everywhere `Value` references are accepted.
//!
//! Supported types are `bool`, `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `f32`,
//! `f64`, `String` and objects (`T: IsA<Object>`).
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::Value;
//!
//! // Value implement From<&i32>, From<&str> and From<Option<&str>>.
//! // Another option is the `ToValue` trait.
//! let mut num = 10.to_value();
//! let mut hello = Value::from("Hello!");
//! let none: Option<&str> = None;
//! let str_none = none.to_value();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//! assert!(hello.is::<String>());
//!
//! // `get` tries to get an optional value of the specified type
//! // and returns an `Err` if the type doesn't match.
//! assert_eq!(num.get(), Ok(10));
//! assert!(num.get::<String>().is_err());
//! assert_eq!(hello.get(), Ok(String::from("Hello!")));
//! assert_eq!(hello.get::<String>(), Ok(String::from("Hello!")));
//! assert_eq!(str_none.get::<Option<String>>(), Ok(None));
//! ```

use libc::{c_char, c_void};
use std::convert::Infallible;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;

use crate::gstring::GString;
use crate::translate::*;
use crate::types::{StaticType, Type};

// rustdoc-stripper-ignore-next
/// A type that can be stored in `Value`s.
pub trait ValueType: ToValue + for<'a> FromValue<'a> + 'static {
    // rustdoc-stripper-ignore-next
    /// Type to get the `Type` from.
    ///
    /// This exists only for handling optional types.
    // FIXME: Should default to Self once associated type defaults are stabilized
    // https://github.com/rust-lang/rust/issues/29661
    type Type: StaticType;
}

// rustdoc-stripper-ignore-next
/// A type that can be stored in `Value`s and is optional.
///
/// These are types were storing an `Option` is valid. Examples are `String` and all object types.
pub trait ValueTypeOptional:
    ValueType + ToValueOptional + FromValueOptional<'static> + StaticType
{
}

impl<T, C> ValueType for Option<T>
where
    T: for<'a> FromValue<'a, Checker = C> + ValueTypeOptional + StaticType + 'static,
    C: ValueTypeChecker<Error = ValueTypeMismatchOrNoneError>,
{
    type Type = T::Type;
}

// rustdoc-stripper-ignore-next
/// Trait for `Value` type checkers.
pub unsafe trait ValueTypeChecker {
    type Error: std::error::Error + Send + Sized + 'static;

    fn check(value: &Value) -> Result<(), Self::Error>;
}

// rustdoc-stripper-ignore-next
/// An error returned from the [`get`](struct.Value.html#method.get) function
/// on a [`Value`](struct.Value.html) for non-optional types an `Option`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValueTypeMismatchError {
    actual: Type,
    requested: Type,
}

impl ValueTypeMismatchError {
    pub fn new(actual: Type, requested: Type) -> Self {
        Self { actual, requested }
    }
}

impl ValueTypeMismatchError {
    pub fn actual_type(&self) -> Type {
        self.actual
    }

    pub fn requested_type(&self) -> Type {
        self.requested
    }
}

impl fmt::Display for ValueTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value type mismatch. Actual {:?}, requested {:?}",
            self.actual_type(),
            self.requested_type(),
        )
    }
}

impl error::Error for ValueTypeMismatchError {}

impl From<Infallible> for ValueTypeMismatchError {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

// rustdoc-stripper-ignore-next
/// Generic `Value` type checker for types.
pub struct GenericValueTypeChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: StaticType> ValueTypeChecker for GenericValueTypeChecker<T> {
    type Error = ValueTypeMismatchError;

    #[doc(alias = "g_type_check_value_holds")]
    fn check(value: &Value) -> Result<(), Self::Error> {
        unsafe {
            if gobject_ffi::g_type_check_value_holds(&value.inner, T::static_type().into_glib())
                == ffi::GFALSE
            {
                Err(ValueTypeMismatchError::new(
                    Type::from_glib(value.inner.g_type),
                    T::static_type(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// An error returned from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html) for optional types.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ValueTypeMismatchOrNoneError {
    WrongValueType(ValueTypeMismatchError),
    UnexpectedNone,
}

impl fmt::Display for ValueTypeMismatchOrNoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WrongValueType(err) => err.fmt(f),
            Self::UnexpectedNone => write!(f, "Unexpected None",),
        }
    }
}

impl error::Error for ValueTypeMismatchOrNoneError {}

impl From<ValueTypeMismatchError> for ValueTypeMismatchOrNoneError {
    fn from(err: ValueTypeMismatchError) -> Self {
        Self::WrongValueType(err)
    }
}

impl From<Infallible> for ValueTypeMismatchOrNoneError {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

// rustdoc-stripper-ignore-next
/// Generic `Value` type checker for optional types.
pub struct GenericValueTypeOrNoneChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: StaticType> ValueTypeChecker for GenericValueTypeOrNoneChecker<T> {
    type Error = ValueTypeMismatchOrNoneError;

    fn check(value: &Value) -> Result<(), Self::Error> {
        GenericValueTypeChecker::<T>::check(value)?;

        unsafe {
            // Values are always zero-initialized so even if pointers are only 32 bits then the
            // whole 64 bit value will be 0 for NULL pointers.
            if value.inner.data[0].v_uint64 == 0 {
                return Err(Self::Error::UnexpectedNone);
            }
        }

        Ok(())
    }
}

// rustdoc-stripper-ignore-next
/// Trait to retrieve the contained value from a `Value`.
///
/// Usually this would not be used directly but from the [`get`](struct.Value.html#method.get)
/// function on a [`Value`](struct.Value.html)
pub unsafe trait FromValue<'a>: Sized {
    // rustdoc-stripper-ignore-next
    /// Value type checker.
    type Checker: ValueTypeChecker;

    // rustdoc-stripper-ignore-next
    /// Get the contained value from a `Value`.
    ///
    /// # Safety
    /// `Self::Checker::check()` must be called first and must not fail.
    unsafe fn from_value(value: &'a Value) -> Self;
}

// rustdoc-stripper-ignore-next
/// Trait for types that implement `FromValue` and are Optional.
///
/// This trait is auto-implemented for the appropriate types and is sealed.
pub trait FromValueOptional<'a>: private::FromValueOptionalSealed<'a> {}

impl<'a, T, C> FromValueOptional<'a> for T
where
    T: FromValue<'a, Checker = C>,
    C: ValueTypeChecker<Error = ValueTypeMismatchOrNoneError>,
{
}

mod private {
    pub trait FromValueOptionalSealed<'a> {}

    impl<'a, T, C> FromValueOptionalSealed<'a> for T
    where
        T: super::FromValue<'a, Checker = C>,
        C: super::ValueTypeChecker<Error = super::ValueTypeMismatchOrNoneError>,
    {
    }
}

// rustdoc-stripper-ignore-next
/// Blanket implementation for all optional types.
unsafe impl<'a, T, C> FromValue<'a> for Option<T>
where
    T: FromValue<'a, Checker = C> + StaticType,
    C: ValueTypeChecker<Error = ValueTypeMismatchOrNoneError>,
{
    type Checker = GenericValueTypeChecker<T>;

    unsafe fn from_value(value: &'a Value) -> Self {
        if let Err(ValueTypeMismatchOrNoneError::UnexpectedNone) = T::Checker::check(value) {
            None
        } else {
            Some(T::from_value(value))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait to convert a value to a `Value`.
///
/// Similar to other common conversion traits, the following invariants are guaranteed:
///
/// - **Invertibility**: `x.to_value().get().unwrap() == x`. In words, [`FromValue`] is the inverse of `ToValue`.
/// - **Idempotence**: `x.to_value() == x.to_value().to_value()`.
///   In words, applying `ToValue` multiple times yields the same result as applying it once.
///   Idempotence also applies the other way around: `value.get::<Value>()` is a no-op.
///
/// There is also the possibility to wrap values within values, see [`BoxedValue`]. All (un-)boxing needs to be done
/// manually, and will be preserved under the conversion methods.
///
/// The conversion methods may cause values to be cloned, which may result in reference counter changes or heap allocations depending
/// on the source and target type.
pub trait ToValue {
    // rustdoc-stripper-ignore-next
    /// Convert a value to a `Value`.
    fn to_value(&self) -> Value;

    // rustdoc-stripper-ignore-next
    /// Returns the type identifer of `self`.
    ///
    /// This is the type of the value to be returned by `to_value`.
    fn value_type(&self) -> Type;
}

// rustdoc-stripper-ignore-next
/// Blanket implementation for all references.
impl<T: ToValue + StaticType> ToValue for &T {
    fn to_value(&self) -> Value {
        T::to_value(*self)
    }

    fn value_type(&self) -> Type {
        T::static_type()
    }
}

// rustdoc-stripper-ignore-next
/// Trait to convert an `Option` to a `Value` for optional types.
pub trait ToValueOptional {
    // rustdoc-stripper-ignore-next
    /// Convert an `Option` to a `Value`.
    #[allow(clippy::wrong_self_convention)]
    fn to_value_optional(s: Option<&Self>) -> Value;
}

// rustdoc-stripper-ignore-next
/// Blanket implementation for all optional types.
impl<T: ToValueOptional + StaticType> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        T::to_value_optional(self.as_ref())
    }

    fn value_type(&self) -> Type {
        T::static_type()
    }
}

impl<T: ToValueOptional + StaticType> StaticType for Option<T> {
    fn static_type() -> Type {
        T::static_type()
    }
}

impl<T: ToValueOptional + StaticType + ?Sized> ToValueOptional for &T {
    fn to_value_optional(s: Option<&Self>) -> Value {
        <T as ToValueOptional>::to_value_optional(s.as_ref().map(|s| **s))
    }
}

unsafe fn copy_value(value: *const gobject_ffi::GValue) -> *mut gobject_ffi::GValue {
    let copy = ffi::g_malloc0(mem::size_of::<gobject_ffi::GValue>()) as *mut gobject_ffi::GValue;
    copy_into_value(copy, value);
    copy
}

unsafe fn free_value(value: *mut gobject_ffi::GValue) {
    clear_value(value);
    ffi::g_free(value as *mut _);
}

unsafe fn init_value(value: *mut gobject_ffi::GValue) {
    ptr::write(value, mem::zeroed());
}

unsafe fn copy_into_value(dest: *mut gobject_ffi::GValue, src: *const gobject_ffi::GValue) {
    gobject_ffi::g_value_init(dest, (*src).g_type);
    gobject_ffi::g_value_copy(src, dest);
}

unsafe fn clear_value(value: *mut gobject_ffi::GValue) {
    // Before GLib 2.48, unsetting a zeroed GValue would give critical warnings
    // https://bugzilla.gnome.org/show_bug.cgi?id=755766
    if (*value).g_type != gobject_ffi::G_TYPE_INVALID {
        gobject_ffi::g_value_unset(value);
    }
}

// TODO: Should use impl !Send for Value {} once stable
crate::wrapper! {
    // rustdoc-stripper-ignore-next
    /// A generic value capable of carrying various types.
    ///
    /// Once created the type of the value can't be changed.
    ///
    /// Some types (e.g. `String` and objects) support `None` values while others
    /// (e.g. numeric types) don't.
    ///
    /// `Value` does not implement the `Send` trait, but [`SendValue`](struct.SendValue.html) can be
    /// used instead.
    ///
    /// See the [module documentation](index.html) for more details.
    #[doc(alias = "GValue")]
    pub struct Value(BoxedInline<gobject_ffi::GValue>);

    match fn {
        copy => |ptr| copy_value(ptr),
        free => |ptr| free_value(ptr),
        init => |ptr| init_value(ptr),
        copy_into => |dest, src| copy_into_value(dest, src),
        clear => |ptr| clear_value(ptr),
    }
}

impl Value {
    // rustdoc-stripper-ignore-next
    /// Creates a new `Value` that is initialized with `type_`
    pub fn from_type(type_: Type) -> Self {
        unsafe {
            assert_eq!(
                gobject_ffi::g_type_check_is_value_type(type_.into_glib()),
                ffi::GTRUE
            );
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, type_.into_glib());
            value
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new `Value` that is initialized for a given `ValueType`.
    pub fn for_value_type<T: ValueType>() -> Self {
        Value::from_type(T::Type::static_type())
    }

    // rustdoc-stripper-ignore-next
    /// Tries to get a value of type `T`.
    ///
    /// Returns `Ok` if the type is correct.
    pub fn get<'a, T>(&'a self) -> Result<T, <<T as FromValue>::Checker as ValueTypeChecker>::Error>
    where
        T: FromValue<'a>,
    {
        unsafe {
            T::Checker::check(self)?;
            Ok(T::from_value(self))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to get a value of an owned type `T`.
    pub fn get_owned<T>(&self) -> Result<T, <<T as FromValue>::Checker as ValueTypeChecker>::Error>
    where
        T: for<'b> FromValue<'b> + 'static,
    {
        unsafe {
            T::Checker::check(self)?;
            Ok(FromValue::from_value(self))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the type of the value corresponds to `T`
    /// or is a sub-type of `T`.
    #[inline]
    pub fn is<T: StaticType>(&self) -> bool {
        self.is_type(T::static_type())
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the type of the value corresponds to `type_`
    /// or is a sub-type of `type_`.
    #[inline]
    pub fn is_type(&self, type_: Type) -> bool {
        self.type_().is_a(type_)
    }

    // rustdoc-stripper-ignore-next
    /// Returns the type of the value.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.inner.g_type) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns whether `Value`s of type `src` can be transformed to type `dst`.
    #[doc(alias = "g_value_type_transformable")]
    pub fn type_transformable(src: Type, dst: Type) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_value_type_transformable(
                src.into_glib(),
                dst.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to transform the value into a value of the target type
    #[doc(alias = "g_value_transform")]
    pub fn transform<T: ValueType>(&self) -> Result<Value, crate::BoolError> {
        self.transform_with_type(T::Type::static_type())
    }

    // rustdoc-stripper-ignore-next
    /// Tries to transform the value into a value of the target type
    #[doc(alias = "g_value_transform")]
    pub fn transform_with_type(&self, type_: Type) -> Result<Value, crate::BoolError> {
        unsafe {
            let mut dest = Value::from_type(type_);
            if from_glib(gobject_ffi::g_value_transform(
                self.to_glib_none().0,
                dest.to_glib_none_mut().0,
            )) {
                Ok(dest)
            } else {
                Err(crate::bool_error!(
                    "Can't transform value of type '{}' into '{}'",
                    self.type_(),
                    type_
                ))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Consumes `Value` and returns the corresponding `GValue`.
    pub fn into_raw(self) -> gobject_ffi::GValue {
        unsafe {
            let s = mem::ManuallyDrop::new(self);
            ptr::read(&s.inner)
        }
    }

    pub fn try_into_send_value<T: Send + StaticType>(self) -> Result<SendValue, Self> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Ok(SendValue::unsafe_from(self.into_raw())) }
        } else {
            Err(self)
        }
    }

    fn content_debug_string(&self) -> GString {
        unsafe { from_glib_full(gobject_ffi::g_strdup_value_contents(self.to_glib_none().0)) }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}) {}", self.type_(), self.content_debug_string())
    }
}

impl<'a, T: ?Sized + ToValue> From<&'a T> for Value {
    #[inline]
    fn from(value: &'a T) -> Self {
        value.to_value()
    }
}

impl From<SendValue> for Value {
    fn from(value: SendValue) -> Self {
        unsafe { Value::unsafe_from(value.into_raw()) }
    }
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }

    fn value_type(&self) -> Type {
        self.type_()
    }
}

impl<'a> ToValue for &'a Value {
    fn to_value(&self) -> Value {
        (*self).clone()
    }

    fn value_type(&self) -> Type {
        self.type_()
    }
}

pub struct NopChecker;

unsafe impl ValueTypeChecker for NopChecker {
    type Error = Infallible;
    fn check(_value: &Value) -> Result<(), Self::Error> {
        Ok(())
    }
}

unsafe impl<'a> FromValue<'a> for Value {
    type Checker = NopChecker;

    unsafe fn from_value(value: &'a Value) -> Self {
        value.clone()
    }
}

unsafe impl<'a> FromValue<'a> for &'a Value {
    type Checker = NopChecker;

    unsafe fn from_value(value: &'a Value) -> Self {
        value
    }
}

impl ToValue for SendValue {
    fn to_value(&self) -> Value {
        unsafe { from_glib_none(self.to_glib_none().0) }
    }

    fn value_type(&self) -> Type {
        self.type_()
    }
}

impl<'a> ToValue for &'a SendValue {
    fn to_value(&self) -> Value {
        unsafe { from_glib_none(self.to_glib_none().0) }
    }

    fn value_type(&self) -> Type {
        self.type_()
    }
}

impl StaticType for BoxedValue {
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_value_get_type()) }
    }
}

crate::wrapper! {
    // rustdoc-stripper-ignore-next
    /// A version of [`Value`](struct.Value.html) for storing `Send` types, that implements Send
    /// itself.
    ///
    /// See the [module documentation](index.html) for more details.
    #[doc(alias = "GValue")]
    pub struct SendValue(BoxedInline<gobject_ffi::GValue>);

    match fn {
        copy => |ptr| copy_value(ptr),
        free => |ptr| free_value(ptr),
        init => |ptr| init_value(ptr),
        copy_into => |dest, src| copy_into_value(dest, src),
        clear => |ptr| clear_value(ptr),
    }
}

unsafe impl Send for SendValue {}

impl SendValue {
    // rustdoc-stripper-ignore-next
    /// Consumes `SendValue` and returns the corresponding `GValue`.
    pub fn into_raw(self) -> gobject_ffi::GValue {
        unsafe {
            let s = mem::ManuallyDrop::new(self);
            ptr::read(&s.inner)
        }
    }
}

impl fmt::Debug for SendValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}) {}", self.type_(), self.content_debug_string())
    }
}

impl Deref for SendValue {
    type Target = Value;

    fn deref(&self) -> &Value {
        unsafe { &*(self as *const SendValue as *const Value) }
    }
}

impl<'a, T: ?Sized + ToSendValue> From<&'a T> for SendValue {
    #[inline]
    fn from(value: &'a T) -> Self {
        value.to_send_value()
    }
}

// rustdoc-stripper-ignore-next
/// Converts to `SendValue`.
pub trait ToSendValue: Send + ToValue {
    // rustdoc-stripper-ignore-next
    /// Returns a `SendValue` clone of `self`.
    fn to_send_value(&self) -> SendValue;
}

impl<T: Send + ToValue + ?Sized> ToSendValue for T {
    fn to_send_value(&self) -> SendValue {
        unsafe { SendValue::unsafe_from(self.to_value().into_raw()) }
    }
}

unsafe impl<'a> FromValue<'a> for &'a str {
    type Checker = GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_string(value.to_glib_none().0);
        CStr::from_ptr(ptr).to_str().expect("Invalid UTF-8")
    }
}

impl ToValue for str {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::from_type(<String>::static_type());

            gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, self.to_glib_full());

            value
        }
    }

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValue for &str {
    fn to_value(&self) -> Value {
        (*self).to_value()
    }

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValueOptional for str {
    fn to_value_optional(s: Option<&Self>) -> Value {
        let mut value = Value::for_value_type::<String>();
        unsafe {
            gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, s.to_glib_full());
        }

        value
    }
}

impl ValueType for String {
    type Type = String;
}

impl ValueTypeOptional for String {}

unsafe impl<'a> FromValue<'a> for String {
    type Checker = GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        String::from(<&str>::from_value(value))
    }
}

impl ToValue for String {
    fn to_value(&self) -> Value {
        <&str>::to_value(&self.as_str())
    }

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl ToValueOptional for String {
    fn to_value_optional(s: Option<&Self>) -> Value {
        <str>::to_value_optional(s.as_ref().map(|s| s.as_str()))
    }
}

impl ValueType for Vec<String> {
    type Type = Vec<String>;
}

unsafe impl<'a> FromValue<'a> for Vec<String> {
    type Checker = GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        FromGlibPtrContainer::from_glib_none(ptr)
    }
}

impl ToValue for Vec<String> {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Self>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl<'a> ToValue for [&'a str] {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Vec<String>>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl<'a> ToValue for &'a [&'a str] {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Vec<String>>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn value_type(&self) -> Type {
        <Vec<String>>::static_type()
    }
}

impl ValueType for bool {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for bool {
    type Checker = GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        from_glib(gobject_ffi::g_value_get_boolean(value.to_glib_none().0))
    }
}

impl ToValue for bool {
    fn to_value(&self) -> Value {
        let mut value = Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_set_boolean(&mut value.inner, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

macro_rules! numeric {
    ($name:ty, $get:expr, $set:expr) => {
        impl ValueType for $name {
            type Type = Self;
        }

        unsafe impl<'a> FromValue<'a> for $name {
            type Checker = GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a Value) -> Self {
                $get(value.to_glib_none().0)
            }
        }

        impl ToValue for $name {
            fn to_value(&self) -> Value {
                let mut value = Value::for_value_type::<Self>();
                unsafe {
                    $set(&mut value.inner, *self);
                }
                value
            }

            fn value_type(&self) -> Type {
                Self::static_type()
            }
        }
    };
}

numeric!(
    i8,
    gobject_ffi::g_value_get_schar,
    gobject_ffi::g_value_set_schar
);
numeric!(
    u8,
    gobject_ffi::g_value_get_uchar,
    gobject_ffi::g_value_set_uchar
);
numeric!(
    i32,
    gobject_ffi::g_value_get_int,
    gobject_ffi::g_value_set_int
);
numeric!(
    u32,
    gobject_ffi::g_value_get_uint,
    gobject_ffi::g_value_set_uint
);
numeric!(
    i64,
    gobject_ffi::g_value_get_int64,
    gobject_ffi::g_value_set_int64
);
numeric!(
    u64,
    gobject_ffi::g_value_get_uint64,
    gobject_ffi::g_value_set_uint64
);
numeric!(
    crate::ILong,
    |v| gobject_ffi::g_value_get_long(v).into(),
    |v, i: crate::ILong| gobject_ffi::g_value_set_long(v, i.0)
);
numeric!(
    crate::ULong,
    |v| gobject_ffi::g_value_get_ulong(v).into(),
    |v, i: crate::ULong| gobject_ffi::g_value_set_ulong(v, i.0)
);
numeric!(
    f32,
    gobject_ffi::g_value_get_float,
    gobject_ffi::g_value_set_float
);
numeric!(
    f64,
    gobject_ffi::g_value_get_double,
    gobject_ffi::g_value_set_double
);

// rustdoc-stripper-ignore-next
/// A [`Value`] containing another [`Value`].
pub struct BoxedValue(pub Value);

impl Deref for BoxedValue {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

impl ValueType for BoxedValue {
    type Type = BoxedValue;
}

unsafe impl<'a> FromValue<'a> for BoxedValue {
    type Checker = GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0);
        BoxedValue(from_glib_none(ptr as *const gobject_ffi::GValue))
    }
}

impl ToValue for BoxedValue {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::from_type(<BoxedValue>::static_type());

            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.0.to_glib_none().0 as ffi::gconstpointer,
            );

            value
        }
    }

    fn value_type(&self) -> Type {
        BoxedValue::static_type()
    }
}

impl ToValueOptional for BoxedValue {
    fn to_value_optional(s: Option<&Self>) -> Value {
        let mut value = Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map(|s| &s.0).to_glib_none().0 as ffi::gconstpointer,
            );
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_value() {
        use std::thread;

        let v = SendValue::from(&1i32);

        // Must compile, while it must fail with Value
        thread::spawn(move || drop(v)).join().unwrap();
    }

    #[test]
    fn test_strv() {
        let v = vec!["123", "456"].to_value();
        assert_eq!(
            v.get::<Vec<GString>>(),
            Ok(vec![GString::from("123"), GString::from("456")])
        );

        let v = vec![String::from("123"), String::from("456")].to_value();
        assert_eq!(
            v.get::<Vec<GString>>(),
            Ok(vec![GString::from("123"), GString::from("456")])
        );
    }

    #[test]
    fn test_from_to_value() {
        let v = 123.to_value();
        assert_eq!(v.get(), Ok(123));
        assert_eq!(
            v.get::<&str>(),
            Err(ValueTypeMismatchError::new(Type::I32, Type::STRING).into())
        );
        assert_eq!(
            v.get::<bool>(),
            Err(ValueTypeMismatchError::new(Type::I32, Type::BOOL))
        );

        // Check if &str / str / Option<&str> etc can be converted and retrieved
        let v_str = "test".to_value();
        assert_eq!(v_str.get::<&str>(), Ok("test"));
        assert_eq!(v_str.get::<Option<&str>>(), Ok(Some("test")));
        assert_eq!(
            v_str.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let some_v = Some("test").to_value();
        assert_eq!(some_v.get::<&str>(), Ok("test"));
        assert_eq!(some_v.get_owned::<String>(), Ok("test".to_string()));
        assert_eq!(
            some_v.get_owned::<Option<String>>(),
            Ok(Some("test".to_string()))
        );
        assert_eq!(some_v.get::<Option<&str>>(), Ok(Some("test")));
        assert_eq!(
            some_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let none_str: Option<&str> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<&str>>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        // Check if owned T and Option<T> can be converted and retrieved
        let v_str = String::from("test").to_value();
        assert_eq!(v_str.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            v_str.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            v_str.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let some_v = Some(String::from("test")).to_value();
        assert_eq!(some_v.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            some_v.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            some_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let none_str: Option<String> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<String>>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        // Check if &T and Option<&T> can be converted and retrieved
        let v_str = (&String::from("test")).to_value();
        assert_eq!(v_str.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            v_str.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            v_str.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let some_v = Some(&String::from("test")).to_value();
        assert_eq!(some_v.get::<String>(), Ok(String::from("test")));
        assert_eq!(
            some_v.get::<Option<String>>(),
            Ok(Some(String::from("test")))
        );
        assert_eq!(
            some_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );

        let none_str: Option<&String> = None;
        let none_v = none_str.to_value();
        assert_eq!(none_v.get::<Option<String>>(), Ok(None));
        assert_eq!(
            none_v.get::<i32>(),
            Err(ValueTypeMismatchError::new(Type::STRING, Type::I32))
        );
    }

    #[test]
    fn test_transform() {
        let v = 123.to_value();
        let v2 = v
            .transform::<String>()
            .expect("Failed to transform to string");
        assert_eq!(v2.get::<&str>(), Ok("123"));
    }

    #[test]
    fn test_into_raw() {
        unsafe {
            let mut v = 123.to_value().into_raw();
            assert_eq!(gobject_ffi::g_type_check_value(&v), ffi::GTRUE);
            assert_eq!(gobject_ffi::g_value_get_int(&v), 123);
            gobject_ffi::g_value_unset(&mut v);
        }
    }

    #[test]
    fn test_debug() {
        fn value_debug_string<T: ToValue>(val: T) -> String {
            format!("{:?}", val.to_value())
        }

        assert_eq!(value_debug_string(1u32), "(guint) 1");
        assert_eq!(value_debug_string(2i32), "(gint) 2");
        assert_eq!(value_debug_string(false), "(gboolean) FALSE");
        assert_eq!(value_debug_string("FooBar"), r#"(gchararray) "FooBar""#);
    }
}
