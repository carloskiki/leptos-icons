#[cfg(feature = "HiLgOutlineArrowSmallUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowSmallUp")]
/// *This icon requires the feature* `HiLgOutlineArrowSmallUp` *to be enabled*.
#[component]
pub fn ArrowSmallUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 19.5L12 4.5M12 4.5L5.25 11.25M12 4.5L18.75 11.25" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}