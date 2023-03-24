#[cfg(feature = "CgFormatHeading")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgFormatHeading")]
/// *This icon requires the feature* `CgFormatHeading` *to be enabled*.
#[component]
pub fn FormatHeading(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6 19V5H8V11H16V5H18V19H16V13H8V19H6Z" fill="currentColor" /></svg>
   }
}