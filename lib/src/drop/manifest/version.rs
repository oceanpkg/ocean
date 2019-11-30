use crate::drop::version::{Version, SemVer};
use super::{Detailed, Flexible};

impl Detailed for Version<'_> {
    type Simple = SemVer;
}

impl From<SemVer> for Flexible<Version<'_>> {
    #[inline]
    fn from(s: SemVer) -> Self {
        Self::Simple(s)
    }
}
