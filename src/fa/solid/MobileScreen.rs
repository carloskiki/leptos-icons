#[cfg(feature = "FaSolidMobileScreen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMobileScreen")]
/// *This icon requires the feature* `FaSolidMobileScreen` *to be enabled*.
#[component]
pub fn MobileScreen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 64C0 28.7 28.7 0 64 0H288c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zM128 448c0 8.8 7.2 16 16 16h64c8.8 0 16-7.2 16-16s-7.2-16-16-16H144c-8.8 0-16 7.2-16 16zM288 64H64V384H288V64z" /></svg>
   }
}