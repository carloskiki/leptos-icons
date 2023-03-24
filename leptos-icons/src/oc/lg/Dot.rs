#[cfg(feature = "OcLgDot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgDot")]
/// *This icon requires the feature* `OcLgDot` *to be enabled*.
#[component]
pub fn Dot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 18a6 6 0 1 1 0-12 6 6 0 0 1 0 12Zm0-1.5a4.5 4.5 0 1 0 0-9 4.5 4.5 0 0 0 0 9Z" /></svg>
   }
}