#[cfg(feature = "HiLgOutlineChevronUpDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronUpDown")]
/// *This icon requires the feature* `HiLgOutlineChevronUpDown` *to be enabled*.
#[component]
pub fn ChevronUpDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M8.25 15L12 18.75L15.75 15M8.25 9L12 5.25L15.75 9" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}