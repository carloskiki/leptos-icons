#[cfg(feature = "TbActivityHeartbeat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbActivityHeartbeat")]
/// *This icon requires the feature* `TbActivityHeartbeat` *to be enabled*.
#[component]
pub fn ActivityHeartbeat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-activity-heartbeat" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12h4.5l1.5 -6l4 12l2 -9l1.5 3h4.5" /></svg>
   }
}