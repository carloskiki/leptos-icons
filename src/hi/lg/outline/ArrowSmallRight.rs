#[cfg(feature = "HiLgOutlineArrowSmallRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowSmallRight")]
/// *This icon requires the feature* `HiLgOutlineArrowSmallRight` *to be enabled*.
#[component]
pub fn ArrowSmallRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 12L19.5 12M19.5 12L12.75 5.25M19.5 12L12.75 18.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}