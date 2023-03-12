#[cfg(feature = "RiMediaLinePause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaLinePause")]
/// *This icon requires the feature* `RiMediaLinePause` *to be enabled*.
#[component]
pub fn Pause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 5h2v14H6V5zm10 0h2v14h-2V5z" /></g></svg>
   }
}