#[cfg(feature = "ImFolderOpen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFolderOpen")]
/// *This icon requires the feature* `ImFolderOpen` *to be enabled*.
#[component]
pub fn FolderOpen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13 15l3-8h-13l-3 8zM2 6l-2 9v-13h4.5l2 2h6.5v2z" /></svg>
   }
}