//! End-user configuration of the local Ocean installation.
//!
//! This is tied heavily to `InstallTarget`.

pub mod file;
pub mod rt;
pub mod user;

#[doc(inline)]
pub use self::{
    file::{ConfigFile, ConfigFileFmt},
    rt::RtConfig,
    user::UserConfig,
};

/// Configuration values that are examined throughout the lifetime of a client
/// program.
///
/// This type supports interior mutability and thus is not [`Sync`]. Note that
/// despite this, it is still [`Send`].
///
/// [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
/// [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
#[derive(Clone, Debug)]
pub struct Config {
    /// User configuration values.
    pub user: UserConfig,
    /// Runtime configuration values.
    pub rt: RtConfig,
}

assert_impl_all!(Config: Send);
assert_not_impl_all!(Config: Sync);

impl Config {
    /// Creates a new instance suitable for using at the start of your program.
    #[inline]
    pub fn create() -> Result<Self, rt::CreateError> {
        let rt = RtConfig::create()?;
        let user = UserConfig::new();
        Ok(Self { user, rt })
    }
}
