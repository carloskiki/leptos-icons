#[cfg(feature = "TbPeace")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPeace")]
/// *This icon requires the feature* `TbPeace` *to be enabled*.
#[component]
pub fn Peace(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-peace" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M12 3l0 18" /><path d="M12 12l6.3 6.3" /><path d="M12 12l-6.3 6.3" /></svg>
   }
}