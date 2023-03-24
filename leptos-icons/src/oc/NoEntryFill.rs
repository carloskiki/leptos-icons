#[cfg(feature = "OcNoEntryFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcNoEntryFill")]
/// *This icon requires the feature* `OcNoEntryFill` *to be enabled*.
#[component]
pub fn NoEntryFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12"><path d="M6 0a6 6 0 1 1 0 12A6 6 0 0 1 6 0Zm3 5H3v2h6Z" /></svg>
   }
}