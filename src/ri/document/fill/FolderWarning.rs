#[cfg(feature = "RiDocumentFillFolderWarning")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillFolderWarning")]
/// *This icon requires the feature* `RiDocumentFillFolderWarning` *to be enabled*.
#[component]
pub fn FolderWarning(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2zM11 9v5h2V9h-2zm0 6v2h2v-2h-2z" /></g></svg>
   }
}