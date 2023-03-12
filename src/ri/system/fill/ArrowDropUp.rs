#[cfg(feature = "RiSystemFillArrowDropUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowDropUp")]
/// *This icon requires the feature* `RiSystemFillArrowDropUp` *to be enabled*.
#[component]
pub fn ArrowDropUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 10l4 4H8z" /></g></svg>
   }
}