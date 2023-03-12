#[cfg(feature = "SiSimpleanalytics")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSimpleanalytics")]
/// *This icon requires the feature* `SiSimpleanalytics` *to be enabled*.
#[component]
pub fn Simpleanalytics(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.019 13.019h3.849V24h-3.85zm8.943-6.68h3.85V24h-3.85zM19.132 0h3.85v24h-3.85z" /></svg>
   }
}