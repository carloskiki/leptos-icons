#[cfg(feature = "TiDivide")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiDivide")]
/// *This icon requires the feature* `TiDivide` *to be enabled*.
#[component]
pub fn Divide(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="6" r="2.25" /><circle cx="12" cy="18" r="2.25" /><path d="M18 10h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12c1.104 0 2-.896 2-2s-.896-2-2-2z" /></svg>
   }
}