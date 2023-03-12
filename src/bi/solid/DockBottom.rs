#[cfg(feature = "BiSolidDockBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDockBottom")]
/// *This icon requires the feature* `BiSolidDockBottom` *to be enabled*.
#[component]
pub fn DockBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2zM19 5l.001 9H5V5h14z" /></svg>
   }
}