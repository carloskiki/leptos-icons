#[cfg(feature = "RiEditorText")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorText")]
/// *This icon requires the feature* `RiEditorText` *to be enabled*.
#[component]
pub fn Text(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13 6v15h-2V6H5V4h14v2z" /></g></svg>
   }
}