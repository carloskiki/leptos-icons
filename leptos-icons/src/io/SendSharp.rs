#[cfg(feature = "IoSendSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSendSharp")]
/// *This icon requires the feature* `IoSendSharp` *to be enabled*.
#[component]
pub fn SendSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M16,464,496,256,16,48V208l320,48L16,304Z" /></svg>
   }
}