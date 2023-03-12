#[cfg(feature = "HiLgOutlineChevronDoubleLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineChevronDoubleLeft")]
/// *This icon requires the feature* `HiLgOutlineChevronDoubleLeft` *to be enabled*.
#[component]
pub fn ChevronDoubleLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M18.75 19.5L11.25 12L18.75 4.5M12.75 19.5L5.25 12L12.75 4.5" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}