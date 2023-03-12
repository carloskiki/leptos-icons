#[cfg(feature = "BiRegularRepeat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRepeat")]
/// *This icon requires the feature* `BiRegularRepeat` *to be enabled*.
#[component]
pub fn Repeat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 6h-5v2h4v9H4V8h5v3l5-4-5-4v3H3a1 1 0 0 0-1 1v11a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1z" /></svg>
   }
}