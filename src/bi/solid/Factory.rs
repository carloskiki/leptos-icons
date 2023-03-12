#[cfg(feature = "BiSolidFactory")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFactory")]
/// *This icon requires the feature* `BiSolidFactory` *to be enabled*.
#[component]
pub fn Factory(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 10V6l-5 4V6l-5 4V4H2v16h20V6l-5 4zm-8 7H7v-3h2v3zm5 0h-2v-3h2v3zm5 0h-2v-3h2v3z" /></svg>
   }
}