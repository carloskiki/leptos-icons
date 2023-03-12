#[cfg(feature = "RiDeviceFillQrScan2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillQrScan2")]
/// *This icon requires the feature* `RiDeviceFillQrScan2` *to be enabled*.
#[component]
pub fn QrScan2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15 3h6v6h-6V3zM9 3v6H3V3h6zm6 18v-6h6v6h-6zm-6 0H3v-6h6v6zM3 11h18v2H3v-2z" /></g></svg>
   }
}