#[cfg(feature = "HiLgOutlineChevronLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronLeft")]
/// *This icon requires the feature* `HiLgOutlineChevronLeft` *to be enabled*.
#[component]
pub fn ChevronLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15.75 19.5L8.25 12L15.75 4.5" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}