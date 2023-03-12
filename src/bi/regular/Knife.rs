#[cfg(feature = "BiRegularKnife")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularKnife")]
/// *This icon requires the feature* `BiRegularKnife` *to be enabled*.
#[component]
pub fn Knife(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19.66 3.6a3 3 0 0 0-4.24 0l-.71.71-7.07 7.07 2.12 2.12-6.36 6.36 1.41 1.42L19.66 6.43c1.1-1.1 1.1-1.73.71-2.12z" /></svg>
   }
}