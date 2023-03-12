#[cfg(feature = "BiRegularDockRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDockRight")]
/// *This icon requires the feature* `BiRegularDockRight` *to be enabled*.
#[component]
pub fn DockRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2zM5 5h9v14H5zm11 14V5h3v14z" /></svg>
   }
}