#[cfg(feature = "BiRegularRightArrow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRightArrow")]
/// *This icon requires the feature* `BiRegularRightArrow` *to be enabled*.
#[component]
pub fn RightArrow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5.536 21.886a1.004 1.004 0 0 0 1.033-.064l13-9a1 1 0 0 0 0-1.644l-13-9A.998.998 0 0 0 5 3v18a1 1 0 0 0 .536.886zM7 4.909 17.243 12 7 19.091V4.909z" /></svg>
   }
}