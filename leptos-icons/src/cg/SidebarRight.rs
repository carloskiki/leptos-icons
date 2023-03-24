#[cfg(feature = "CgSidebarRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSidebarRight")]
/// *This icon requires the feature* `CgSidebarRight` *to be enabled*.
#[component]
pub fn SidebarRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 4H17V20H3V4ZM5 6H15V18H5V6Z" fill="currentColor" /><path d="M21 4H19V20H21V4Z" fill="currentColor" /></svg>
   }
}