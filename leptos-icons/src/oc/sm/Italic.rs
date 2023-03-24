#[cfg(feature = "OcSmItalic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmItalic")]
/// *This icon requires the feature* `OcSmItalic` *to be enabled*.
#[component]
pub fn Italic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M6 2.75A.75.75 0 0 1 6.75 2h6.5a.75.75 0 0 1 0 1.5h-2.505l-3.858 9H9.25a.75.75 0 0 1 0 1.5h-6.5a.75.75 0 0 1 0-1.5h2.505l3.858-9H6.75A.75.75 0 0 1 6 2.75Z" /></svg>
   }
}