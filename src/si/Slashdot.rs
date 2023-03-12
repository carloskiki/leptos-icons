#[cfg(feature = "SiSlashdot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSlashdot")]
/// *This icon requires the feature* `SiSlashdot` *to be enabled*.
#[component]
pub fn Slashdot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M19.777 0L7.037 24H1.868L14.605 0h5.172zm2.354 19.801c0 2.268-1.841 4.105-4.109 4.105s-4.107-1.838-4.107-4.105 1.839-4.107 4.107-4.107 4.109 1.839 4.109 4.107z" /></svg>
   }
}