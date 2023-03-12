#[cfg(feature = "BiRegularArrowToRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowToRight")]
/// *This icon requires the feature* `BiRegularArrowToRight` *to be enabled*.
#[component]
pub fn ArrowToRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 6h2v12h-2zM4 13h8.586l-4.293 4.293 1.414 1.414L16.414 12 9.707 5.293 8.293 6.707 12.586 11H4z" /></svg>
   }
}