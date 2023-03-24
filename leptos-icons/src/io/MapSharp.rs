#[cfg(feature = "IoMapSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMapSharp")]
/// *This icon requires the feature* `IoMapSharp` *to be enabled*.
#[component]
pub fn MapSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M327.71,130.93,184,39,32,144V480l152.29-98.93L328,473,480,368V32ZM312,421,200,349V91l112,72Z" /></svg>
   }
}