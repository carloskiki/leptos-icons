#[cfg(feature = "IoBowlingBallOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBowlingBallOutline")]
/// *This icon requires the feature* `IoBowlingBallOutline` *to be enabled*.
#[component]
pub fn BowlingBallOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><circle cx="256" cy="256" r="208" fill="none" stroke="#000" stroke-miterlimit="10" stroke-width="32" /><circle cx="288" cy="200" r="24" /><circle cx="296" cy="128" r="24" /><circle cx="360" cy="168" r="24" /></svg>
   }
}