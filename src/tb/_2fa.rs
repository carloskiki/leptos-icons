#[cfg(feature = "Tb2fa")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "Tb2fa")]
/// *This icon requires the feature* `Tb2fa` *to be enabled*.
#[component]
pub fn _2fa(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-2fa" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 16h-4l3.47 -4.66a2 2 0 1 0 -3.47 -1.54" /><path d="M10 16v-8h4" /><path d="M10 12l3 0" /><path d="M17 16v-6a2 2 0 0 1 4 0v6" /><path d="M17 13l4 0" /></svg>
   }
}