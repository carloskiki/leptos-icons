#[cfg(feature = "HiLgOutlineArrowSmallDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowSmallDown")]
/// *This icon requires the feature* `HiLgOutlineArrowSmallDown` *to be enabled*.
#[component]
pub fn ArrowSmallDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 4.5V19.5M12 19.5L18.75 12.75M12 19.5L5.25 12.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}