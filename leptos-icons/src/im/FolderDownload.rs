#[cfg(feature = "ImFolderDownload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFolderDownload")]
/// *This icon requires the feature* `ImFolderDownload` *to be enabled*.
#[component]
pub fn FolderDownload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 4l-2-2h-7v13h16v-11h-7zM8 13.5l-3.5-3.5h2.5v-4h2v4h2.5l-3.5 3.5z" /></svg>
   }
}