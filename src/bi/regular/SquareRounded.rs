#[cfg(feature = "BiRegularSquareRounded")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSquareRounded")]
/// *This icon requires the feature* `BiRegularSquareRounded` *to be enabled*.
#[component]
pub fn SquareRounded(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 2H7C4.243 2 2 4.243 2 7v10c0 2.757 2.243 5 5 5h10c2.757 0 5-2.243 5-5V7c0-2.757-2.243-5-5-5zm3 15c0 1.654-1.346 3-3 3H7c-1.654 0-3-1.346-3-3V7c0-1.654 1.346-3 3-3h10c1.654 0 3 1.346 3 3v10z" /></svg>
   }
}