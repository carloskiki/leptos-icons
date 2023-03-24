#[cfg(feature = "TbSignalLte")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSignalLte")]
/// *This icon requires the feature* `TbSignalLte` *to be enabled*.
#[component]
pub fn SignalLte(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-signal-lte" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 8h-4v8h4" /><path d="M17 12h2.5" /><path d="M4 8v8h4" /><path d="M10 8h4" /><path d="M12 8v8" /></svg>
   }
}