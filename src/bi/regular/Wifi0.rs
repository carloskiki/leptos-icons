#[cfg(feature = "BiRegularWifi0")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularWifi0")]
/// *This icon requires the feature* `BiRegularWifi0` *to be enabled*.
#[component]
pub fn Wifi0(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="18" r="2" /></svg>
   }
}