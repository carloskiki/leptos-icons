#[cfg(feature = "FaSolidMobileButton")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMobileButton")]
/// *This icon requires the feature* `FaSolidMobileButton` *to be enabled*.
#[component]
pub fn MobileButton(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H288c35.3 0 64-28.7 64-64V64c0-35.3-28.7-64-64-64H64zM176 400a32 32 0 1 1 0 64 32 32 0 1 1 0-64z" /></svg>
   }
}