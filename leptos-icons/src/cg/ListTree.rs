#[cfg(feature = "CgListTree")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgListTree")]
/// *This icon requires the feature* `CgListTree` *to be enabled*.
#[component]
pub fn ListTree(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M9 1H1V9H9V6H11V20H15V23H23V15H15V18H13V6H15V9H23V1H15V4H9V1ZM21 3H17V7H21V3ZM17 17H21V21H17V17Z" fill="currentColor" /></svg>
   }
}