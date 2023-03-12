#[cfg(feature = "HiLgOutlineBars3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineBars3")]
/// *This icon requires the feature* `HiLgOutlineBars3` *to be enabled*.
#[component]
pub fn Bars3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3.75 6.75H20.25M3.75 12H20.25M3.75 17.25H20.25" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}