#[cfg(feature = "BiSolidAlarmExclamation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidAlarmExclamation")]
/// *This icon requires the feature* `BiSolidAlarmExclamation` *to be enabled*.
#[component]
pub fn AlarmExclamation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m17.284 3.707 1.412-1.416 3.01 3-1.413 1.417zm-10.586 0-2.99 2.999L2.29 5.294l2.99-3zM12 4c-4.879 0-9 4.121-9 9s4.121 9 9 9 9-4.121 9-9-4.121-9-9-9zm1 14h-2v-2h2v2zm0-4h-2V8h2v6z" /></svg>
   }
}