#[cfg(feature = "CgAlignBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgAlignBottom")]
/// *This icon requires the feature* `CgAlignBottom` *to be enabled*.
#[component]
pub fn AlignBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M13 10H17V16H13V10Z" fill="currentColor" fill-opacity="0.5" /><path d="M11 4H7V16H11V4Z" fill="currentColor" /><path d="M18 18H6V20H18V18Z" fill="currentColor" /></svg>
   }
}