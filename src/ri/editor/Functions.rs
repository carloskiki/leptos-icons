#[cfg(feature = "RiEditorFunctions")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorFunctions")]
/// *This icon requires the feature* `RiEditorFunctions` *to be enabled*.
#[component]
pub fn Functions(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M5 18l7.68-6L5 6V4h14v2H8.263L16 12l-7.737 6H19v2H5v-2z" /></g></svg>
   }
}