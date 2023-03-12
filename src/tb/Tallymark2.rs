#[cfg(feature = "TbTallymark2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbTallymark2")]
/// *This icon requires the feature* `TbTallymark2` *to be enabled*.
#[component]
pub fn Tallymark2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-tallymark-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 5l0 14" /><path d="M14 5l0 14" /></svg>
   }
}