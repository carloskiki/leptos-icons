#[cfg(feature = "HiLgOutlineHashtag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineHashtag")]
/// *This icon requires the feature* `HiLgOutlineHashtag` *to be enabled*.
#[component]
pub fn Hashtag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5.25 8.25H20.25M3.75 15.75H18.75M16.95 2.25L13.05 21.75M10.9503 2.25L7.05029 21.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}