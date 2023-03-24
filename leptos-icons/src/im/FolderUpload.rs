#[cfg(feature = "ImFolderUpload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFolderUpload")]
/// *This icon requires the feature* `ImFolderUpload` *to be enabled*.
#[component]
pub fn FolderUpload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 4l-2-2h-7v13h16v-11h-7zM8 7.5l3.5 3.5h-2.5v4h-2v-4h-2.5l3.5-3.5z" /></svg>
   }
}