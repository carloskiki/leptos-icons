#[cfg(feature = "BiSolidObjectsVerticalBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidObjectsVerticalBottom")]
/// *This icon requires the feature* `BiSolidObjectsVerticalBottom` *to be enabled*.
#[component]
pub fn ObjectsVerticalBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M2 20h20v2H2z" /><rect x="5" y="2" width="6" height="16" rx="1" /><rect x="13" y="6" width="6" height="12" rx="1" /></svg>
   }
}