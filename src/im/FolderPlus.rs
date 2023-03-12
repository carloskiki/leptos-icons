#[cfg(feature = "ImFolderPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFolderPlus")]
/// *This icon requires the feature* `ImFolderPlus` *to be enabled*.
#[component]
pub fn FolderPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 4l-2-2h-7v13h16v-11h-7zM11 11h-2v2h-2v-2h-2v-2h2v-2h2v2h2v2z" /></svg>
   }
}