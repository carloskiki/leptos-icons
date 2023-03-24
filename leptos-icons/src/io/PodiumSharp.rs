#[cfg(feature = "IoPodiumSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPodiumSharp")]
/// *This icon requires the feature* `IoPodiumSharp` *to be enabled*.
#[component]
pub fn PodiumSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="160" y="32" width="192" height="448" /><rect x="384" y="192" width="112" height="288" /><rect x="16" y="128" width="112" height="352" /></svg>
   }
}