#[cfg(feature = "RiEditorAlignBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorAlignBottom")]
/// *This icon requires the feature* `RiEditorAlignBottom` *to be enabled*.
#[component]
pub fn AlignBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 19h18v2H3v-2zm5-6h3l-4 4-4-4h3V3h2v10zm10 0h3l-4 4-4-4h3V3h2v10z" /></g></svg>
   }
}