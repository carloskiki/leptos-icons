#[cfg(feature = "HiLgOutlineArrowSmallLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowSmallLeft")]
/// *This icon requires the feature* `HiLgOutlineArrowSmallLeft` *to be enabled*.
#[component]
pub fn ArrowSmallLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19.5 12L4.5 12M4.5 12L11.25 18.75M4.5 12L11.25 5.25" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}