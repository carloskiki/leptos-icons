#[cfg(feature = "BiSolidMehAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMehAlt")]
/// *This icon requires the feature* `BiSolidMehAlt` *to be enabled*.
#[component]
pub fn MehAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-6 8h4v2H6v-2zm10 7H7.974v-2H16v2zm2-5h-4v-2h4v2z" /></svg>
   }
}