#[cfg(feature = "TbClockHour9")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbClockHour9")]
/// *This icon requires the feature* `TbClockHour9` *to be enabled*.
#[component]
pub fn ClockHour9(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-clock-hour-9" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M12 12h-3.5" /><path d="M12 7v5" /></svg>
   }
}