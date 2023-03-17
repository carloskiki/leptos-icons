#[cfg(feature = "BiSolidAlarmSnooze")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidAlarmSnooze")]
/// *This icon requires the feature* `BiSolidAlarmSnooze` *to be enabled*.
#[component]
pub fn AlarmSnooze(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m17.284 3.707 1.412-1.416 3.01 3-1.413 1.417zm-10.586 0-2.99 2.999L2.29 5.294l2.99-3zM12 4c-4.878 0-9 4.121-9 9s4.122 9 9 9c4.879 0 9-4.121 9-9s-4.121-9-9-9zm4 13H8.131l4-6H8V9h7.868l-1.035 1.554-.001.001L11.869 15H16v2z" /></svg>
   }
}