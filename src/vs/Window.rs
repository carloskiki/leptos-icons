#[cfg(feature = "VsWindow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsWindow")]
/// *This icon requires the feature* `VsWindow` *to be enabled*.
#[component]
pub fn Window(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14.5 2h-13l-.5.5v11l.5.5h13l.5-.5v-11l-.5-.5zM14 13H2V6h12v7zm0-8H2V3h12v2z" /></svg>
   }
}