#[cfg(feature = "HiLgOutlineArrowRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowRight")]
/// *This icon requires the feature* `HiLgOutlineArrowRight` *to be enabled*.
#[component]
pub fn ArrowRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M13.5 4.5L21 12M21 12L13.5 19.5M21 12H3" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}