#[cfg(feature = "BiRegularDotsHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDotsHorizontal")]
/// *This icon requires the feature* `BiRegularDotsHorizontal` *to be enabled*.
#[component]
pub fn DotsHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 10h4v4h-4zm6 0h4v4h-4zM4 10h4v4H4z" /></svg>
   }
}