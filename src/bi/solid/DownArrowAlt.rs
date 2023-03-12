#[cfg(feature = "BiSolidDownArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDownArrowAlt")]
/// *This icon requires the feature* `BiSolidDownArrowAlt` *to be enabled*.
#[component]
pub fn DownArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 12h-5V6h-2v6H6l6 7z" /></svg>
   }
}