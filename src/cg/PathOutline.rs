#[cfg(feature = "CgPathOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathOutline")]
/// *This icon requires the feature* `CgPathOutline` *to be enabled*.
#[component]
pub fn PathOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M5 5H15V9H19V19H9V15H5V5ZM7 7H13V9H9V13H7V7ZM11 17H17V11H15V15H11V17ZM13 11H11V13H13V11Z" fill="currentColor" /></svg>
   }
}