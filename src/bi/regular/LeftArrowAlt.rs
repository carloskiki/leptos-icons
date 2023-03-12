#[cfg(feature = "BiRegularLeftArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLeftArrowAlt")]
/// *This icon requires the feature* `BiRegularLeftArrowAlt` *to be enabled*.
#[component]
pub fn LeftArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.707 17.293 8.414 13H18v-2H8.414l4.293-4.293-1.414-1.414L4.586 12l6.707 6.707z" /></svg>
   }
}