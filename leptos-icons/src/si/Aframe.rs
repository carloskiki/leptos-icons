#[cfg(feature = "SiAframe")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAframe")]
/// *This icon requires the feature* `SiAframe` *to be enabled*.
#[component]
pub fn Aframe(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M17.37 17.07H6.57L4.24 24H3.01l8.23-24h1.52l8.23 24h-1.3zm-.39-1.13l-5-14.96-5.03 14.98h10.03Z" /></svg>
   }
}