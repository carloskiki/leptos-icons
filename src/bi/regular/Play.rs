#[cfg(feature = "BiRegularPlay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPlay")]
/// *This icon requires the feature* `BiRegularPlay` *to be enabled*.
#[component]
pub fn Play(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 6v12l10-6z" /></svg>
   }
}