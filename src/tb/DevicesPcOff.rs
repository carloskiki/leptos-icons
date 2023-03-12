#[cfg(feature = "TbDevicesPcOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDevicesPcOff")]
/// *This icon requires the feature* `TbDevicesPcOff` *to be enabled*.
#[component]
pub fn DevicesPcOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-devices-pc-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 9v10h-6v-14h2" /><path d="M13 9h9v7h-2m-4 0h-4v-4" /><path d="M14 19h5" /><path d="M17 17v2" /><path d="M6 13v.01" /><path d="M6 16v.01" /><path d="M3 3l18 18" /></svg>
   }
}