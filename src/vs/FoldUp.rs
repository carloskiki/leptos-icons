#[cfg(feature = "VsFoldUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsFoldUp")]
/// *This icon requires the feature* `VsFoldUp` *to be enabled*.
#[component]
pub fn FoldUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1 7.4l.7.7 6-6 6 6 .7-.7L8.1 1h-.7L1 7.4zm0 6l.7.7 6-6 6 6 .7-.7L8.1 7h-.7L1 13.4z" /></svg>
   }
}