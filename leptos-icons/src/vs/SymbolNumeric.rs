#[cfg(feature = "VsSymbolNumeric")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolNumeric")]
/// *This icon requires the feature* `VsSymbolNumeric` *to be enabled*.
#[component]
pub fn SymbolNumeric(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M11 1v4h4v1h-4v4h4v1h-4v4h-1v-4H6v4H5v-4H1v-1h4V6H1V5h4V1h1v4h4V1h1zM6 6v4h4V6H6z" /></svg>
   }
}