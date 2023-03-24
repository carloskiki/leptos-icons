#[cfg(feature = "IoFilterSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFilterSharp")]
/// *This icon requires the feature* `IoFilterSharp` *to be enabled*.
#[component]
pub fn FilterSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="16" y="120" width="480" height="48" /><rect x="96" y="232" width="320" height="48" /><rect x="192" y="344" width="128" height="48" /></svg>
   }
}