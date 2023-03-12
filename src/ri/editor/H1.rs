#[cfg(feature = "RiEditorH1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorH1")]
/// *This icon requires the feature* `RiEditorH1` *to be enabled*.
#[component]
pub fn H1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M13 20h-2v-7H4v7H2V4h2v7h7V4h2v16zm8-12v12h-2v-9.796l-2 .536V8.67L19.5 8H21z" /></g></svg>
   }
}