#[cfg(feature = "ImFolderMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFolderMinus")]
/// *This icon requires the feature* `ImFolderMinus` *to be enabled*.
#[component]
pub fn FolderMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 4l-2-2h-7v13h16v-11h-7zM11 11h-6v-2h6v2z" /></svg>
   }
}