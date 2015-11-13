pub use super::ext::arith::*;
pub use super::ext::boolean::*;
pub use super::ext::complex::*;
pub use super::ext::error::*;
pub use super::ext::memory::*;
pub use super::ext::print::*;
pub use super::ext::random::*;
pub use super::ext::utils::*;
pub use super::ext::rs::*;

#[link(name = "R")]
extern {
    pub fn R_FlushConsole() -> ();
    pub fn R_ProcessEvents() -> ();
}
