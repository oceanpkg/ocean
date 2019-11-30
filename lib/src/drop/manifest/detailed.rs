//! Allows for flexibility in parsing simple or detailed information for the
//! same type.

/// A type that has detailed information.
pub trait Detailed: Sized {
    /// The basic version of this type.
    type Simple;
}

impl<'a, D: Detailed> Detailed for &'a D {
    type Simple = &'a D::Simple;
}

impl<'a, D: Detailed> Detailed for &'a mut D {
    type Simple = &'a mut D::Simple;
}

/// A type that can either be parsed as simple or detailed information.
#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Flexible<D: Detailed> {
    /// The minimal amount of information that is within `D`.
    Simple(D::Simple),
    /// All information stored within `D`.
    Detailed(D),
}

impl<D: Detailed> From<D> for Flexible<D> {
    #[inline]
    fn from(d: D) -> Self {
        Self::Detailed(d)
    }
}

// TODO: Implement `From<D::Simple>` for `Flexible<D>` when specialization is
// made stable. See https://github.com/rust-lang/rust/issues/31844.

impl<A, B> PartialEq<Flexible<B>> for Flexible<A>
where
    A: Detailed + PartialEq<B>,
    B: Detailed,
    <A as Detailed>::Simple: PartialEq<<B as Detailed>::Simple>,
{
    #[inline]
    fn eq(&self, other: &Flexible<B>) -> bool {
        use self::Flexible::*;
        match (self, other) {
            (Simple(a),   Simple(b))   => a == b,
            (Detailed(a), Detailed(b)) => a == b,
            _ => false,
        }
    }
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

    /// Returns a new `Flexible` containing a shared reference to the data of
    /// `self`.
    #[inline]
    pub fn as_ref(&self) -> Flexible<&D> {
        use self::Flexible::*;
        match self {
            Simple(s)   => Simple(s),
            Detailed(d) => Detailed(d),
        }
    }

    /// Returns a new `Flexible` containing a mutable reference to the data of
    /// `self`.
    #[inline]
    pub fn as_mut(&mut self) -> Flexible<&mut D> {
        use self::Flexible::*;
        match self {
            Simple(s)   => Simple(s),
            Detailed(d) => Detailed(d),
        }
    }
}
