#[cfg(feature = "TbPlayerSkipForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPlayerSkipForward")]
/// *This icon requires the feature* `TbPlayerSkipForward` *to be enabled*.
#[component]
pub fn PlayerSkipForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-player-skip-forward" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 5v14l12 -7z" /><path d="M20 5l0 14" /></svg>
   }
}