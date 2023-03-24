#[cfg(feature = "SiOkta")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiOkta")]
/// *This icon requires the feature* `SiOkta` *to be enabled*.
#[component]
pub fn Okta(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0C5.389 0 0 5.35 0 12s5.35 12 12 12 12-5.35 12-12S18.611 0 12 0zm0 18c-3.325 0-6-2.675-6-6s2.675-6 6-6 6 2.675 6 6-2.675 6-6 6z" /></svg>
   }
}