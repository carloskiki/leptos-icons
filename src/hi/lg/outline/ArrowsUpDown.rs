#[cfg(feature = "HiLgOutlineArrowsUpDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowsUpDown")]
/// *This icon requires the feature* `HiLgOutlineArrowsUpDown` *to be enabled*.
#[component]
pub fn ArrowsUpDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 7.5L7.5 3M7.5 3L12 7.5M7.5 3V16.5M21 16.5L16.5 21M16.5 21L12 16.5M16.5 21L16.5 7.5" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}