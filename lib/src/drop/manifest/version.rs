use crate::drop::version::{Version, SemVer};
use super::Detailed;

impl Detailed for Version<'_> {
    type Simple = SemVer;
}
