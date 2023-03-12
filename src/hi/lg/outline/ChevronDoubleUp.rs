#[cfg(feature = "HiLgOutlineChevronDoubleUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronDoubleUp")]
/// *This icon requires the feature* `HiLgOutlineChevronDoubleUp` *to be enabled*.
#[component]
pub fn ChevronDoubleUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 12.75L12 5.25L19.5 12.75M4.5 18.75L12 11.25L19.5 18.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}