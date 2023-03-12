#[cfg(feature = "RiSystemLineArrowDropRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineArrowDropRight")]
/// *This icon requires the feature* `RiSystemLineArrowDropRight` *to be enabled*.
#[component]
pub fn ArrowDropRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.172 12L9.343 9.172l1.414-1.415L15 12l-4.243 4.243-1.414-1.415z" /></g></svg>
   }
}