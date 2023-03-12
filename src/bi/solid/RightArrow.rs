#[cfg(feature = "BiSolidRightArrow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidRightArrow")]
/// *This icon requires the feature* `BiSolidRightArrow` *to be enabled*.
#[component]
pub fn RightArrow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5.536 21.886a1.004 1.004 0 0 0 1.033-.064l13-9a1 1 0 0 0 0-1.644l-13-9A1 1 0 0 0 5 3v18a1 1 0 0 0 .536.886z" /></svg>
   }
}