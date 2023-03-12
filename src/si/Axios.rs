#[cfg(feature = "SiAxios")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAxios")]
/// *This icon requires the feature* `SiAxios` *to be enabled*.
#[component]
pub fn Axios(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.0683 2.89968V22.2973l-2.11399 1.70265V7.8638H4.975l6.0933-4.96412zM14.93426 0v15.76724H19.025l-6.20044 5.08865V1.4689L14.93426 0z" /></svg>
   }
}