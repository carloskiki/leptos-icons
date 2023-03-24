#[cfg(feature = "IoGridSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGridSharp")]
/// *This icon requires the feature* `IoGridSharp` *to be enabled*.
#[component]
pub fn GridSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M240,240H32V32H240Z" /><path d="M480,240H272V32H480Z" /><path d="M240,480H32V272H240Z" /><path d="M480,480H272V272H480Z" /></svg>
   }
}