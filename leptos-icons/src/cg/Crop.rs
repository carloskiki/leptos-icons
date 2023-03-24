#[cfg(feature = "CgCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgCrop")]
/// *This icon requires the feature* `CgCrop` *to be enabled*.
#[component]
pub fn Crop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M7.93164 9.00891V16H15V20.0089H17V16H20.9316V14H17V7.00891H9.93164V3H7.93164V7.00891H4V9.00891H7.93164ZM9.93164 9.00891V14H15V9.00891H9.93164Z" fill="currentColor" /></svg>
   }
}