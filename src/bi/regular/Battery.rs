#[cfg(feature = "BiRegularBattery")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBattery")]
/// *This icon requires the feature* `BiRegularBattery` *to be enabled*.
#[component]
pub fn Battery(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 18h14c1.103 0 2-.897 2-2v-2h2v-4h-2V8c0-1.103-.897-2-2-2H4c-1.103 0-2 .897-2 2v8c0 1.103.897 2 2 2zM4 8h14l.002 8H4V8z" /></svg>
   }
}