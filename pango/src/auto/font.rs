// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
use crate::Context;
use crate::Coverage;
use crate::FontDescription;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
use crate::FontFace;
use crate::FontMap;
use crate::FontMetrics;
use crate::Glyph;
use crate::Language;
use crate::Rectangle;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
use std::ptr;

glib::wrapper! {
    #[doc(alias = "PangoFont")]
    pub struct Font(Object<ffi::PangoFont, ffi::PangoFontClass>);

    match fn {
        type_ => || ffi::pango_font_get_type(),
    }
}

impl Font {
    pub const NONE: Option<&'static Font> = None;

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_font_deserialize")]
    pub fn deserialize(
        context: &Context,
        bytes: &glib::Bytes,
    ) -> Result<Option<Font>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::pango_font_deserialize(
                context.to_glib_none().0,
                bytes.to_glib_none().0,
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

pub trait FontExt: 'static {
    #[doc(alias = "pango_font_describe")]
    fn describe(&self) -> Option<FontDescription>;

    #[doc(alias = "pango_font_describe_with_absolute_size")]
    fn describe_with_absolute_size(&self) -> Option<FontDescription>;

    #[doc(alias = "pango_font_get_coverage")]
    #[doc(alias = "get_coverage")]
    fn coverage(&self, language: &Language) -> Option<Coverage>;

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_font_get_face")]
    #[doc(alias = "get_face")]
    fn face(&self) -> Option<FontFace>;

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    //#[doc(alias = "pango_font_get_features")]
    //#[doc(alias = "get_features")]
    //fn features(&self, features: /*Unimplemented*/&mut Fundamental: Pointer, num_features: &mut u32) -> u32;

    #[doc(alias = "pango_font_get_font_map")]
    #[doc(alias = "get_font_map")]
    fn font_map(&self) -> Option<FontMap>;

    #[doc(alias = "pango_font_get_glyph_extents")]
    #[doc(alias = "get_glyph_extents")]
    fn glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle);

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    //#[doc(alias = "pango_font_get_hb_font")]
    //#[doc(alias = "get_hb_font")]
    //fn hb_font(&self) -> /*Ignored*/Option<harf_buzz::font_t>;

    #[doc(alias = "pango_font_get_metrics")]
    #[doc(alias = "get_metrics")]
    fn metrics(&self, language: Option<&Language>) -> Option<FontMetrics>;

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_font_has_char")]
    fn has_char(&self, wc: char) -> bool;

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_font_serialize")]
    fn serialize(&self) -> Option<glib::Bytes>;
}

impl<O: IsA<Font>> FontExt for O {
    fn describe(&self) -> Option<FontDescription> {
        unsafe { from_glib_full(ffi::pango_font_describe(self.as_ref().to_glib_none().0)) }
    }

    fn describe_with_absolute_size(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(ffi::pango_font_describe_with_absolute_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn coverage(&self, language: &Language) -> Option<Coverage> {
        unsafe {
            from_glib_full(ffi::pango_font_get_coverage(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    fn face(&self) -> Option<FontFace> {
        unsafe { from_glib_none(ffi::pango_font_get_face(self.as_ref().to_glib_none().0)) }
    }

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    //fn features(&self, features: /*Unimplemented*/&mut Fundamental: Pointer, num_features: &mut u32) -> u32 {
    //    unsafe { TODO: call ffi:pango_font_get_features() }
    //}

    fn font_map(&self) -> Option<FontMap> {
        unsafe { from_glib_none(ffi::pango_font_get_font_map(self.as_ref().to_glib_none().0)) }
    }

    fn glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_font_get_glyph_extents(
                self.as_ref().to_glib_none().0,
                glyph,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    //fn hb_font(&self) -> /*Ignored*/Option<harf_buzz::font_t> {
    //    unsafe { TODO: call ffi:pango_font_get_hb_font() }
    //}

    fn metrics(&self, language: Option<&Language>) -> Option<FontMetrics> {
        unsafe {
            from_glib_full(ffi::pango_font_get_metrics(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    fn has_char(&self, wc: char) -> bool {
        unsafe {
            from_glib(ffi::pango_font_has_char(
                self.as_ref().to_glib_none().0,
                wc.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    fn serialize(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_full(ffi::pango_font_serialize(self.as_ref().to_glib_none().0)) }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Font")
    }
}
