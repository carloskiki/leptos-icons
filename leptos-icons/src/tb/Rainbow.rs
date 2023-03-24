#[cfg(feature = "TbRainbow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRainbow")]
/// *This icon requires the feature* `TbRainbow` *to be enabled*.
#[component]
pub fn Rainbow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-rainbow" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M22 17c0 -5.523 -4.477 -10 -10 -10s-10 4.477 -10 10" /><path d="M18 17a6 6 0 1 0 -12 0" /><path d="M14 17a2 2 0 1 0 -4 0" /></svg>
   }
}