#[cfg(feature = "VsFold")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsFold")]
/// *This icon requires the feature* `VsFold` *to be enabled*.
#[component]
pub fn Fold(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M11.952 2.381L7.976 6.357 4 2.381 3.38 3l4.286 4.285h.619L12.57 3l-.618-.619zM3.904 14l4.072-4.072L12.047 14l.62-.619L8.284 9h-.619l-4.381 4.381.619.619z" /></svg>
   }
}