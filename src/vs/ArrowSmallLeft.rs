#[cfg(feature = "VsArrowSmallLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowSmallLeft")]
/// *This icon requires the feature* `VsArrowSmallLeft` *to be enabled*.
#[component]
pub fn ArrowSmallLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M6.5 10.7L4 8.2v-.7L6.5 5l.71.7-1.64 1.65h5.57v1H5.57L7.22 10l-.72.7z" /></svg>
   }
}