#[cfg(feature = "RiEditorAlignLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorAlignLeft")]
/// *This icon requires the feature* `RiEditorAlignLeft` *to be enabled*.
#[component]
pub fn AlignLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 4h18v2H3V4zm0 15h14v2H3v-2zm0-5h18v2H3v-2zm0-5h14v2H3V9z" /></g></svg>
   }
}