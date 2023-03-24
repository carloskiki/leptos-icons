#[cfg(feature = "TbSquarePlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSquarePlus")]
/// *This icon requires the feature* `TbSquarePlus` *to be enabled*.
#[component]
pub fn SquarePlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-square-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" /><path d="M9 12l6 0" /><path d="M12 9l0 6" /></svg>
   }
}