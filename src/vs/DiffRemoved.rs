#[cfg(feature = "VsDiffRemoved")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDiffRemoved")]
/// *This icon requires the feature* `VsDiffRemoved` *to be enabled*.
#[component]
pub fn DiffRemoved(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M10 7v1H5V7h5z" /><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 1h12l.5.5v12l-.5.5h-12l-.5-.5v-12l.5-.5zM2 13h11V2H2v11z" /></svg>
   }
}