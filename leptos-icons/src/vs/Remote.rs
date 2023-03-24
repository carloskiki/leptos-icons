#[cfg(feature = "VsRemote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRemote")]
/// *This icon requires the feature* `VsRemote` *to be enabled*.
#[component]
pub fn Remote(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M12.904 9.57L8.928 5.596l3.976-3.976-.619-.62L8 5.286v.619l4.285 4.285.62-.618zM3 5.62l4.072 4.07L3 13.763l.619.618L8 10v-.619L3.619 5 3 5.619z" /></svg>
   }
}