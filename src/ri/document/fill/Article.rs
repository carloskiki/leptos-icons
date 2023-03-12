#[cfg(feature = "RiDocumentFillArticle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillArticle")]
/// *This icon requires the feature* `RiDocumentFillArticle` *to be enabled*.
#[component]
pub fn Article(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M20 22H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1zM7 6v4h4V6H7zm0 6v2h10v-2H7zm0 4v2h10v-2H7zm6-9v2h4V7h-4z" /></g></svg>
   }
}