#[cfg(feature = "RiEditorNumber1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorNumber1")]
/// *This icon requires the feature* `RiEditorNumber1` *to be enabled*.
#[component]
pub fn Number1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14 1.5V22h-2V3.704L7.5 4.91V2.839l5-1.339z" /></g></svg>
   }
}