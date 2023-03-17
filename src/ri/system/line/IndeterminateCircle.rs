#[cfg(feature = "RiSystemLineIndeterminateCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineIndeterminateCircle")]
/// *This icon requires the feature* `RiSystemLineIndeterminateCircle` *to be enabled*.
#[component]
pub fn IndeterminateCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm0-2a8 8 0 1 0 0-16 8 8 0 0 0 0 16zm-5-9h10v2H7v-2z" /></g></svg>
   }
}