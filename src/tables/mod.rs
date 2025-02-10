#[cfg(feature = "aces")]
pub mod aces_to_spectrum;

#[cfg(feature = "dci_p3")]
pub mod dci_p3_to_spectrum;

#[cfg(feature = "rec2020")]
pub mod rec2020_to_spectrum;

#[cfg(feature = "srgb")]
pub mod srgb_to_spectrum;
