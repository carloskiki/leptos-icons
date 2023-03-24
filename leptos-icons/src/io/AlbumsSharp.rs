#[cfg(feature = "IoAlbumsSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAlbumsSharp")]
/// *This icon requires the feature* `IoAlbumsSharp` *to be enabled*.
#[component]
pub fn AlbumsSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="128" y="64" width="256" height="32" /><rect x="96" y="112" width="320" height="32" /><path d="M464,448H48V160H464Z" /></svg>
   }
}