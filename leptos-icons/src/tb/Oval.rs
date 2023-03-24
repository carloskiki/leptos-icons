#[cfg(feature = "TbOval")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbOval")]
/// *This icon requires the feature* `TbOval` *to be enabled*.
#[component]
pub fn Oval(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-oval" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-6 0a6 9 0 1 0 12 0a6 9 0 1 0 -12 0" /></svg>
   }
}