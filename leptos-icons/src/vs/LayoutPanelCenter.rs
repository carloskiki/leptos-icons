#[cfg(feature = "VsLayoutPanelCenter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutPanelCenter")]
/// *This icon requires the feature* `VsLayoutPanelCenter` *to be enabled*.
#[component]
pub fn LayoutPanelCenter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H4V14H2ZM5 10V2H11V10H5ZM12 2H14V14H12V2Z" /></svg>
   }
}