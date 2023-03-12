#[cfg(feature = "RiEditorHeading")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorHeading")]
/// *This icon requires the feature* `RiEditorHeading` *to be enabled*.
#[component]
pub fn Heading(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17 11V4h2v17h-2v-8H7v8H5V4h2v7z" /></g></svg>
   }
}