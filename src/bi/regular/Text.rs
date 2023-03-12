#[cfg(feature = "BiRegularText")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularText")]
/// *This icon requires the feature* `BiRegularText` *to be enabled*.
#[component]
pub fn Text(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 8h2V6h3.252L7.68 18H5v2h8v-2h-2.252L13.32 6H17v2h2V4H5z" /></svg>
   }
}