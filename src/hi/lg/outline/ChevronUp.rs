#[cfg(feature = "HiLgOutlineChevronUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronUp")]
/// *This icon requires the feature* `HiLgOutlineChevronUp` *to be enabled*.
#[component]
pub fn ChevronUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 15.75L12 8.25L19.5 15.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}