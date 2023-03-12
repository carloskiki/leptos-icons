#[cfg(feature = "RiSystemLineArrowLeftS")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineArrowLeftS")]
/// *This icon requires the feature* `RiSystemLineArrowLeftS` *to be enabled*.
#[component]
pub fn ArrowLeftS(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10.828 12l4.95 4.95-1.414 1.414L8 12l6.364-6.364 1.414 1.414z" /></g></svg>
   }
}