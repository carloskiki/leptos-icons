#[cfg(feature = "TbWifi1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbWifi1")]
/// *This icon requires the feature* `TbWifi1` *to be enabled*.
#[component]
pub fn Wifi1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-wifi-1" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 18l.01 0" /><path d="M9.172 15.172a4 4 0 0 1 5.656 0" /></svg>
   }
}