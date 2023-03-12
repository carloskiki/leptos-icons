#[cfg(feature = "BiSolidCheckbox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCheckbox")]
/// *This icon requires the feature* `BiSolidCheckbox` *to be enabled*.
#[component]
pub fn Checkbox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 19h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2z" /></svg>
   }
}