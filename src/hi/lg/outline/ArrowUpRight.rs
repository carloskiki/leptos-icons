#[cfg(feature = "HiLgOutlineArrowUpRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowUpRight")]
/// *This icon requires the feature* `HiLgOutlineArrowUpRight` *to be enabled*.
#[component]
pub fn ArrowUpRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 19.5L19.5 4.5M19.5 4.5L8.25 4.5M19.5 4.5V15.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}