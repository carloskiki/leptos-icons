#[cfg(feature = "BiRegularBody")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBody")]
/// *This icon requires the feature* `BiRegularBody` *to be enabled*.
#[component]
pub fn Body(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="4" r="2" /><path d="M15 22V9h5V7H4v2h5v13h2v-7h2v7z" /></svg>
   }
}