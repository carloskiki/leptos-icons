#[cfg(feature = "IoTriangle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTriangle")]
/// *This icon requires the feature* `IoTriangle` *to be enabled*.
#[component]
pub fn Triangle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M464,464H48a16,16,0,0,1-14.07-23.62l208-384a16,16,0,0,1,28.14,0l208,384A16,16,0,0,1,464,464Z" /></svg>
   }
}