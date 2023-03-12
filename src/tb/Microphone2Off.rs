#[cfg(feature = "TbMicrophone2Off")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMicrophone2Off")]
/// *This icon requires the feature* `TbMicrophone2Off` *to be enabled*.
#[component]
pub fn Microphone2Off(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-microphone-2-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M16.908 12.917a5 5 0 1 0 -5.827 -5.819" /><path d="M10.116 10.125l-6.529 7.46a2 2 0 1 0 2.827 2.83l7.461 -6.529" /><path d="M3 3l18 18" /></svg>
   }
}