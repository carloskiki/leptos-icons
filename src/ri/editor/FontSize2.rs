#[cfg(feature = "RiEditorFontSize2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorFontSize2")]
/// *This icon requires the feature* `RiEditorFontSize2` *to be enabled*.
#[component]
pub fn FontSize2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 6v15H8V6H2V4h14v2h-6zm8 8v7h-2v-7h-3v-2h8v2h-3z" /></g></svg>
   }
}