#[cfg(feature = "CgSpectrum")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSpectrum")]
/// *This icon requires the feature* `CgSpectrum` *to be enabled*.
#[component]
pub fn Spectrum(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 16H16C16 11.5817 12.4183 8 8 8V12C10.2091 12 12 13.7909 12 16Z" fill="currentColor" /></svg>
   }
}