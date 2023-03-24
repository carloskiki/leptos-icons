#[cfg(feature = "CgAlignCenter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgAlignCenter")]
/// *This icon requires the feature* `CgAlignCenter` *to be enabled*.
#[component]
pub fn AlignCenter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9 13H15V17H9V13Z" fill="currentColor" fill-opacity="0.5" /><path d="M6 7H18V11H6V7Z" fill="currentColor" /></svg>
   }
}