//! Translation key infrastructure.

use std::ffi::CStr;

pub trait I18nKey: Copy + std::fmt::Debug {
    fn en(self) -> &'static str;
    fn en_cstr(self) -> &'static CStr;
}

#[macro_export]
macro_rules! i18n_keys {
    ($vis:vis $EnumName:ident { $($(#[$comment:meta])* $variant:ident = $en:literal;)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $EnumName { $($variant,)* }

        impl $EnumName {
            pub fn en(self) -> &'static str { match self { $($EnumName::$variant => $en,)* } }
            pub fn en_cstr(self) -> &'static CStr {
                match self { $($EnumName::$variant => {
                    let bytes: &[u8] = concat!($en, "\0").as_bytes();
                    unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
                }),* }
            }
            pub fn all() -> &'static [$EnumName] { &[$($EnumName::$variant,)*] }
        }

        impl $crate::i18n::i18n_keys::I18nKey for $EnumName {
            fn en(self) -> &'static str { self.en() }
            fn en_cstr(self) -> &'static CStr { self.en_cstr() }
        }
    };
}
