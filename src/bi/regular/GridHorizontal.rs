#[cfg(feature = "BiRegularGridHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularGridHorizontal")]
/// *This icon requires the feature* `BiRegularGridHorizontal` *to be enabled*.
#[component]
pub fn GridHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 7h4v4h-4zm6 0h4v4h-4zM4 7h4v4H4zm6 6h4v4h-4zm6 0h4v4h-4zM4 13h4v4H4z" /></svg>
   }
}