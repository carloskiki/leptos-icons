#[cfg(feature = "HiLgOutlineArrowUpLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowUpLeft")]
/// *This icon requires the feature* `HiLgOutlineArrowUpLeft` *to be enabled*.
#[component]
pub fn ArrowUpLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19.5 19.5L4.5 4.5M4.5 4.5L4.5 15.75M4.5 4.5L15.75 4.5" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}