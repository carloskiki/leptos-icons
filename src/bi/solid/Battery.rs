#[cfg(feature = "BiSolidBattery")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBattery")]
/// *This icon requires the feature* `BiSolidBattery` *to be enabled*.
#[component]
pub fn Battery(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 8a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2h2v-4h-2V8z" /></svg>
   }
}