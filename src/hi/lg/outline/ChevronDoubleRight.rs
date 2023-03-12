#[cfg(feature = "HiLgOutlineChevronDoubleRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronDoubleRight")]
/// *This icon requires the feature* `HiLgOutlineChevronDoubleRight` *to be enabled*.
#[component]
pub fn ChevronDoubleRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11.25 4.5L18.75 12L11.25 19.5M5.25 4.5L12.75 12L5.25 19.5" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}