#[cfg(feature = "VsLayoutPanelRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutPanelRight")]
/// *This icon requires the feature* `VsLayoutPanelRight` *to be enabled*.
#[component]
pub fn LayoutPanelRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1 2L2 1H14L15 2V14L14 15H2L1 14V2ZM2 2V14H5V2H2ZM6 2V10H14V2H6Z" /></svg>
   }
}