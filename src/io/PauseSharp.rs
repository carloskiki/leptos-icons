#[cfg(feature = "IoPauseSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPauseSharp")]
/// *This icon requires the feature* `IoPauseSharp` *to be enabled*.
#[component]
pub fn PauseSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M224,432H144V80h80Z" /><path d="M368,432H288V80h80Z" /></svg>
   }
}