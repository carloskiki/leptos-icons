#[cfg(feature = "IoClose")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoClose")]
/// *This icon requires the feature* `IoClose` *to be enabled*.
#[component]
pub fn Close(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M289.94,256l95-95A24,24,0,0,0,351,127l-95,95-95-95A24,24,0,0,0,127,161l95,95-95,95A24,24,0,1,0,161,385l95-95,95,95A24,24,0,0,0,385,351Z" /></svg>
   }
}