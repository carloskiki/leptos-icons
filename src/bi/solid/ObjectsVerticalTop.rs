#[cfg(feature = "BiSolidObjectsVerticalTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidObjectsVerticalTop")]
/// *This icon requires the feature* `BiSolidObjectsVerticalTop` *to be enabled*.
#[component]
pub fn ObjectsVerticalTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M2 2h20v2H2z" /><rect x="5" y="6" width="6" height="16" rx="1" /><rect x="13" y="6" width="6" height="12" rx="1" /></svg>
   }
}