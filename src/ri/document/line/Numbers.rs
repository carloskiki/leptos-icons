#[cfg(feature = "RiDocumentLineNumbers")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineNumbers")]
/// *This icon requires the feature* `RiDocumentLineNumbers` *to be enabled*.
#[component]
pub fn Numbers(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 18H4v-8h5v8zm-2-2v-4H6v4h1zm6 0V8h-1v8h1zm2 2h-5V6h5v12zm4-2V4h-1v12h1zm2 2h-5V2h5v16zm1 4H3v-2h19v2z" /></g></svg>
   }
}