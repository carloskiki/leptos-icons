#[cfg(feature = "TbAlignLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbAlignLeft")]
/// *This icon requires the feature* `TbAlignLeft` *to be enabled*.
#[component]
pub fn AlignLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-align-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 6l16 0" /><path d="M4 12l10 0" /><path d="M4 18l14 0" /></svg>
   }
}