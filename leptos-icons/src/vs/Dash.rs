#[cfg(feature = "VsDash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDash")]
/// *This icon requires the feature* `VsDash` *to be enabled*.
#[component]
pub fn Dash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M5 8h6v1H5z" /></svg>
   }
}