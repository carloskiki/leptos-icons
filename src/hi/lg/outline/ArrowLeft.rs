#[cfg(feature = "HiLgOutlineArrowLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowLeft")]
/// *This icon requires the feature* `HiLgOutlineArrowLeft` *to be enabled*.
#[component]
pub fn ArrowLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M10.5 19.5L3 12M3 12L10.5 4.5M3 12H21" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}