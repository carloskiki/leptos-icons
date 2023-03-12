#[cfg(feature = "HiLgOutlineChevronRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronRight")]
/// *This icon requires the feature* `HiLgOutlineChevronRight` *to be enabled*.
#[component]
pub fn ChevronRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M8.25 4.5L15.75 12L8.25 19.5" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}