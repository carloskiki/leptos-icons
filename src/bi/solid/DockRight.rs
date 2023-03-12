#[cfg(feature = "BiSolidDockRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDockRight")]
/// *This icon requires the feature* `BiSolidDockRight` *to be enabled*.
#[component]
pub fn DockRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5zM5 5h9v14H5V5z" /></svg>
   }
}