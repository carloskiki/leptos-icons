#[cfg(feature = "HiLgOutlineMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineMinus")]
/// *This icon requires the feature* `HiLgOutlineMinus` *to be enabled*.
#[component]
pub fn Minus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19.5 12L4.5 12" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}