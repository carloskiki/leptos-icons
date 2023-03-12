#[cfg(feature = "VsLayoutSidebarLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutSidebarLeft")]
/// *This icon requires the feature* `VsLayoutSidebarLeft` *to be enabled*.
#[component]
pub fn LayoutSidebarLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM14 14H7V2H14V14Z" /></svg>
   }
}