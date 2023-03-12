#[cfg(feature = "TbLamp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLamp")]
/// *This icon requires the feature* `TbLamp` *to be enabled*.
#[component]
pub fn Lamp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-lamp" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 20h6" /><path d="M12 20v-8" /><path d="M5 12h14l-4 -8h-6z" /></svg>
   }
}