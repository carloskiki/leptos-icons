#[cfg(feature = "CgAlignLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgAlignLeft")]
/// *This icon requires the feature* `CgAlignLeft` *to be enabled*.
#[component]
pub fn AlignLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M8 13H14V17H8V13Z" fill="currentColor" fill-opacity="0.5" /><path d="M6 6H4V18H6V6Z" fill="currentColor" /><path d="M20 7H8V11H20V7Z" fill="currentColor" /></svg>
   }
}