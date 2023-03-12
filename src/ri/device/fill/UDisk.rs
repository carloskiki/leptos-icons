#[cfg(feature = "RiDeviceFillUDisk")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillUDisk")]
/// *This icon requires the feature* `RiDeviceFillUDisk` *to be enabled*.
#[component]
pub fn UDisk(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4 12h16a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-8a1 1 0 0 1 1-1zM5 2h14v8H5V2zm4 3v2h2V5H9zm4 0v2h2V5h-2z" /></g></svg>
   }
}