#[cfg(feature = "HiLgOutlineChevronDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronDown")]
/// *This icon requires the feature* `HiLgOutlineChevronDown` *to be enabled*.
#[component]
pub fn ChevronDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19.5 8.25L12 15.75L4.5 8.25" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}