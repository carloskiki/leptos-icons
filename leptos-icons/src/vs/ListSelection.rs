#[cfg(feature = "VsListSelection")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsListSelection")]
/// *This icon requires the feature* `VsListSelection` *to be enabled*.
#[component]
pub fn ListSelection(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1 12v-1h9v1H1zm0-5h14v1H1V7zm11-4v1H1V3h11z" /></svg>
   }
}