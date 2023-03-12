#[cfg(feature = "BiRegularReflectVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularReflectVertical")]
/// *This icon requires the feature* `BiRegularReflectVertical` *to be enabled*.
#[component]
pub fn ReflectVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m3 18 6-6-6-6v12zm12-6 6 6V6l-6 6zm-4-9h2v3h-2zm0 5h2v3h-2zm0 5h2v3h-2zm0 5h2v3h-2z" /></svg>
   }
}