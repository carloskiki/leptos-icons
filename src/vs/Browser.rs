#[cfg(feature = "VsBrowser")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsBrowser")]
/// *This icon requires the feature* `VsBrowser` *to be enabled*.
#[component]
pub fn Browser(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 1h13l.5.5v12l-.5.5h-13l-.5-.5v-12l.5-.5zM2 5v8h12V5H2zm0-1h12V2H2v2z" /></svg>
   }
}