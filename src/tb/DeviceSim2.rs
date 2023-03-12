#[cfg(feature = "TbDeviceSim2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceSim2")]
/// *This icon requires the feature* `TbDeviceSim2` *to be enabled*.
#[component]
pub fn DeviceSim2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-sim-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 3h8.5l4.5 4.5v12.5a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1z" /><path d="M10 9h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /></svg>
   }
}