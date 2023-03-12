#[cfg(feature = "RiSystemLineArrowDropUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineArrowDropUp")]
/// *This icon requires the feature* `RiSystemLineArrowDropUp` *to be enabled*.
#[component]
pub fn ArrowDropUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 11.828l-2.828 2.829-1.415-1.414L12 9l4.243 4.243-1.415 1.414L12 11.828z" /></g></svg>
   }
}