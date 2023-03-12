#[cfg(feature = "SiFacebookgaming")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFacebookgaming")]
/// *This icon requires the feature* `SiFacebookgaming` *to be enabled*.
#[component]
pub fn Facebookgaming(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v24h15.67v-7.35H7.35v-9.3H24V0zm8.33 15.68h8.32V24H24V8.32H8.33Z" /></svg>
   }
}