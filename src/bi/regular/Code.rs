#[cfg(feature = "BiRegularCode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCode")]
/// *This icon requires the feature* `BiRegularCode` *to be enabled*.
#[component]
pub fn Code(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8.293 6.293 2.586 12l5.707 5.707 1.414-1.414L5.414 12l4.293-4.293zm7.414 11.414L21.414 12l-5.707-5.707-1.414 1.414L18.586 12l-4.293 4.293z" /></svg>
   }
}