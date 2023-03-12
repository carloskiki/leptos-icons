#[cfg(feature = "HiLgOutlineBolt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineBolt")]
/// *This icon requires the feature* `HiLgOutlineBolt` *to be enabled*.
#[component]
pub fn Bolt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3.75 13.5L14.25 2.25L12 10.5H20.25L9.75 21.75L12 13.5H3.75Z" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}