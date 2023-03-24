#[cfg(feature = "IoStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoStop")]
/// *This icon requires the feature* `IoStop` *to be enabled*.
#[component]
pub fn Stop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M392,432H120a40,40,0,0,1-40-40V120a40,40,0,0,1,40-40H392a40,40,0,0,1,40,40V392A40,40,0,0,1,392,432Z" /></svg>
   }
}