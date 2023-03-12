#[cfg(feature = "CgFormatUppercase")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgFormatUppercase")]
/// *This icon requires the feature* `CgFormatUppercase` *to be enabled*.
#[component]
pub fn FormatUppercase(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M13 9H10V17H8V9H5V7H13V9ZM18 13H16V17H14V13H12V11H18V13Z" fill="currentColor" /></svg>
   }
}