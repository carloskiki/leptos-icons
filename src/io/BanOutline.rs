#[cfg(feature = "IoBanOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBanOutline")]
/// *This icon requires the feature* `IoBanOutline` *to be enabled*.
#[component]
pub fn BanOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><circle cx="256" cy="256" r="208" fill="none" stroke="#000" stroke-miterlimit="10" stroke-width="32" /><line x1="108.92" y1="108.92" x2="403.08" y2="403.08" fill="none" stroke="#000" stroke-miterlimit="10" stroke-width="32" /></svg>
   }
}