#[cfg(feature = "TbSeparator")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSeparator")]
/// *This icon requires the feature* `TbSeparator` *to be enabled*.
#[component]
pub fn Separator(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-separator" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12l0 .01" /><path d="M7 12l10 0" /><path d="M21 12l0 .01" /></svg>
   }
}