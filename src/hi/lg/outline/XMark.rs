#[cfg(feature = "HiLgOutlineXMark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineXMark")]
/// *This icon requires the feature* `HiLgOutlineXMark` *to be enabled*.
#[component]
pub fn XMark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6 18L18 6M6 6L18 18" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}