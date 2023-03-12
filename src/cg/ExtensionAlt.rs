#[cfg(feature = "CgExtensionAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgExtensionAlt")]
/// *This icon requires the feature* `CgExtensionAlt` *to be enabled*.
#[component]
pub fn ExtensionAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M5 5V19H13V13H19V5H5ZM11 7H7V11H11V7ZM11 13H7V17H11V13ZM13 11H17V7H13V11Z" fill="currentColor" /></svg>
   }
}