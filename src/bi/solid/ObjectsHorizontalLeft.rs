#[cfg(feature = "BiSolidObjectsHorizontalLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidObjectsHorizontalLeft")]
/// *This icon requires the feature* `BiSolidObjectsHorizontalLeft` *to be enabled*.
#[component]
pub fn ObjectsHorizontalLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M2 2h2v20H2z" /><rect x="6" y="13" width="16" height="6" rx="1" /><rect x="6" y="5" width="12" height="6" rx="1" /></svg>
   }
}