#[cfg(feature = "CgEditStraight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgEditStraight")]
/// *This icon requires the feature* `CgEditStraight` *to be enabled*.
#[component]
pub fn EditStraight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 4C15.866 4 19 7.13401 19 11H5C5 7.13401 8.13401 4 12 4Z" fill="currentColor" /><path d="M5 13H1V11H5V13Z" fill="currentColor" /><path d="M19 13C19 16.866 15.866 20 12 20C8.13401 20 5 16.866 5 13H19Z" fill="currentColor" /><path d="M19 13V11H23V13H19Z" fill="currentColor" /></svg>
   }
}