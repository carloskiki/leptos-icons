#[cfg(feature = "HiLgOutlineCube")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineCube")]
/// *This icon requires the feature* `HiLgOutlineCube` *to be enabled*.
#[component]
pub fn Cube(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M21 7.5L12 2.25L3 7.5M21 7.5L12 12.75M21 7.5V16.5L12 21.75M3 7.5L12 12.75M3 7.5V16.5L12 21.75M12 12.75V21.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}