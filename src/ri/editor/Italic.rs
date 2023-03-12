#[cfg(feature = "RiEditorItalic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorItalic")]
/// *This icon requires the feature* `RiEditorItalic` *to be enabled*.
#[component]
pub fn Italic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15 20H7v-2h2.927l2.116-12H9V4h8v2h-2.927l-2.116 12H15z" /></g></svg>
   }
}