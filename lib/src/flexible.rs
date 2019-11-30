/// Defines a `struct` or `enum` such that it can flexibly be parsed via
/// `serde::Deserialize` as either:
/// - Only the first field/variant type
/// - Key/value pairs
macro_rules! flexible {
    // Parse struct
    (
        $(#[serde($($t_serde:meta),+ $(,)?)])*
        $(#[$t_meta:meta])*
        $t_vis:vis struct $t:ident $(<$l:lifetime>)? {
            // The field used for the `Simple` form.
            $(#[doc = $s_doc:literal])+
            $(#[serde($($s_serde:meta),+ $(,)?)])*
            $s_vis:vis $s:ident: $s_ty:ty,

            $(
                $(#[doc = $f_doc:literal])+
                $(#[serde($($f_serde:meta),+ $(,)?)])*
                $f_vis:vis $f:ident: $f_ty:ty,
            )+
        }
    ) => {
        // The actual struct definition.
        $(#[$t_meta])*
        $t_vis struct $t $(<$l>)? {
            $(#[doc = $s_doc])+
            $s_vis $s: $s_ty,

            $(
                $(#[doc = $f_doc])+
                $f_vis $f: $f_ty,
            )*
        }

        impl<'de $(: $l)? $(, $l)?> serde::Deserialize<'de> for $t $(<$l>)? {
            #[inline]
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                use crate::flexible::{Flexible, Detailed};

                // Wrapper type is in a module so that it can have the same
                // name as the original.
                mod wrapper {
                    #[allow(unused_imports)]
                    use super::*;

                    $(#[serde($($t_serde),+)])*
                    #[derive(Deserialize)]
                    pub struct $t $(<$l>)? {
                        $(#[serde($($s_serde),+)])*
                        #[allow(unused)]
                        $s: $s_ty,

                        $(
                            $(#[serde($($f_serde),+)])*
                            #[allow(unused)]
                            $f: $f_ty,
                        )*
                    }
                }

                impl $(<$l>)? Detailed for wrapper::$t $(<$l>)? {
                    type Simple = $s_ty;
                }

                impl $(<$l>)? Detailed for $t $(<$l>)? {
                    type Simple = $s_ty;
                }

                Flexible::<wrapper::$t>::deserialize(de)
                    .map(|wrapper| unsafe {
                        std::mem::transmute::<_, Flexible<$t>>(wrapper)
                    })
                    .map(Flexible::into_detailed)
            }
        }
    };
    // Parse enum
    (
        $(#[serde($($t_serde:meta),+ $(,)?)])*
        $(#[$t_meta:meta])*
        $t_vis:vis enum $t:ident $(<$l:lifetime>)? {
            // The variant used for the `Simple` form.
            $(#[doc = $s_doc:literal])+
            $(#[serde($($s_serde:meta),+ $(,)?)])*
            $s:ident ($s_ty:ty),

            $(
                $(#[doc = $v_doc:literal])+
                $(#[serde($($v_serde:meta),+ $(,)?)])*
                $v:ident ($v_ty:ty),
            )+
        }
    ) => {
        // The actual enum definition.
        $(#[$t_meta])*
        $t_vis enum $t $(<$l>)? {
            $(#[doc = $s_doc])+
            $s($s_ty),

            $(
                $(#[doc = $v_doc])+
                $v($v_ty),
            )*
        }

        impl<'de $(: $l)? $(, $l)?> serde::Deserialize<'de> for $t $(<$l>)? {
            #[inline]
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                use crate::flexible::{Flexible, Detailed};

                // Wrapper type is in a module so that it can have the same
                // name as the original.
                mod wrapper {
                    #[allow(unused_imports)]
                    use super::*;

                    $(#[serde($($t_serde),+)])*
                    #[derive(Deserialize)]
                    $t_vis enum $t $(<$l>)? {
                        $(#[serde($($s_serde),+)])*
                        #[allow(unused)]
                        $s($s_ty),

                        $(
                            $(#[serde($($v_serde),+)])*
                            #[allow(unused)]
                            $v($v_ty),
                        )*
                    }
                }

                impl $(<$l>)? Detailed for wrapper::$t $(<$l>)? {
                    type Simple = $s_ty;
                }

                impl $(<$l>)? Detailed for $t $(<$l>)? {
                    type Simple = $s_ty;
                }

                Flexible::<wrapper::$t>::deserialize(de)
                    .map(|wrapper| unsafe {
                        std::mem::transmute::<_, Flexible<$t>>(wrapper)
                    })
                    .map(Flexible::into_detailed)
            }
        }
    };
}

/// A type that has detailed information.
pub(crate) trait Detailed: Sized {
    /// The basic version of this type.
    type Simple;
}

/// A type that can either be parsed as simple or detailed information.
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum Flexible<D: Detailed> {
    /// The minimal amount of information that is within `D`.
    Simple(D::Simple),
    /// All information stored within `D`.
    Detailed(D),
}

impl<D: Detailed> Flexible<D> {
    /// Converts `self` into the detailed form `D` so that all information can
    /// be used in a simple way without extra `match`ing.
    #[inline]
    pub fn into_detailed(self) -> D where D::Simple: Into<D> {
        match self {
            Self::Simple(s)   => s.into(),
            Self::Detailed(d) => d,
        }
    }
}
