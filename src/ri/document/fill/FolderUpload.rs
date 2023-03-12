#[cfg(feature = "RiDocumentFillFolderUpload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillFolderUpload")]
/// *This icon requires the feature* `RiDocumentFillFolderUpload` *to be enabled*.
#[component]
pub fn FolderUpload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2zM13 13h3l-4-4-4 4h3v4h2v-4z" /></g></svg>
   }
}