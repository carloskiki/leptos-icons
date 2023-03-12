#[cfg(feature = "FiUpload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiUpload")]
/// *This icon requires the feature* `FiUpload` *to be enabled*.
#[component]
pub fn Upload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline points="17 8 12 3 7 8" /><line x1="12" y1="3" x2="12" y2="15" /></svg>
   }
}