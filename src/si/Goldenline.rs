#[cfg(feature = "SiGoldenline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiGoldenline")]
/// *This icon requires the feature* `SiGoldenline` *to be enabled*.
#[component]
pub fn Goldenline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.997 24a11.995 11.995 0 0 0 11.949-13.04h-6.781v2.943h1.226a6.667 6.667 0 1 1-.114-4.156h5.509A11.995 11.995 0 1 0 12 23.991z" /></svg>
   }
}