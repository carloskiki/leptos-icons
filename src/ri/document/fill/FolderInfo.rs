#[cfg(feature = "RiDocumentFillFolderInfo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillFolderInfo")]
/// *This icon requires the feature* `RiDocumentFillFolderInfo` *to be enabled*.
#[component]
pub fn FolderInfo(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2zM11 9v2h2V9h-2zm0 3v5h2v-5h-2z" /></g></svg>
   }
}