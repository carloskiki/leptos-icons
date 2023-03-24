#[cfg(feature = "SiFramer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFramer")]
/// *This icon requires the feature* `SiFramer` *to be enabled*.
#[component]
pub fn Framer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M4 0h16v8h-8zM4 8h8l8 8H4zM4 16h8v8z" /></svg>
   }
}