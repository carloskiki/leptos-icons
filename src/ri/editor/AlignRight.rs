#[cfg(feature = "RiEditorAlignRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorAlignRight")]
/// *This icon requires the feature* `RiEditorAlignRight` *to be enabled*.
#[component]
pub fn AlignRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 4h18v2H3V4zm4 15h14v2H7v-2zm-4-5h18v2H3v-2zm4-5h14v2H7V9z" /></g></svg>
   }
}