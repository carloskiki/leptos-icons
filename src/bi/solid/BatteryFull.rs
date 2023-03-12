#[cfg(feature = "BiSolidBatteryFull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBatteryFull")]
/// *This icon requires the feature* `BiSolidBatteryFull` *to be enabled*.
#[component]
pub fn BatteryFull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 18h14a2 2 0 0 0 2-2v-2h2v-4h-2V8a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2zm1-9h12v6H5V9z" /></svg>
   }
}