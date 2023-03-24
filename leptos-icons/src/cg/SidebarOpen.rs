#[cfg(feature = "CgSidebarOpen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSidebarOpen")]
/// *This icon requires the feature* `CgSidebarOpen` *to be enabled*.
#[component]
pub fn SidebarOpen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 4H21V20H3V4ZM9 6H19V18H9V6Z" fill="currentColor" /></svg>
   }
}