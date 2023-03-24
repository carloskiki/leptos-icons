#[cfg(feature = "SiPurism")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPurism")]
/// *This icon requires the feature* `SiPurism` *to be enabled*.
#[component]
pub fn Purism(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 19.588H0V4.412h24zM2.824 16.765h18.352v-9.53H2.824Z" /></svg>
   }
}