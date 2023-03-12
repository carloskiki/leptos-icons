#[cfg(feature = "TbAxisX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbAxisX")]
/// *This icon requires the feature* `TbAxisX` *to be enabled*.
#[component]
pub fn AxisX(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-axis-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 13v.01" /><path d="M4 9v.01" /><path d="M4 5v.01" /><path d="M17 20l3 -3l-3 -3" /><path d="M4 17h16" /></svg>
   }
}