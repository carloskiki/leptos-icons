#[cfg(feature = "RiDocumentFillNumbers")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillNumbers")]
/// *This icon requires the feature* `RiDocumentFillNumbers` *to be enabled*.
#[component]
pub fn Numbers(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 18H4v-8h5v8zm6 0h-5V6h5v12zm6 0h-5V2h5v16zm1 4H3v-2h19v2z" /></g></svg>
   }
}