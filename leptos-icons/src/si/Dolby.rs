#[cfg(feature = "SiDolby")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiDolby")]
/// *This icon requires the feature* `SiDolby` *to be enabled*.
#[component]
pub fn Dolby(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24,20.352V3.648H0v16.704H24z M18.433,5.806h2.736v12.387h-2.736c-2.839,0-5.214-2.767-5.214-6.194S15.594,5.806,18.433,5.806z M2.831,5.806h2.736c2.839,0,5.214,2.767,5.214,6.194s-2.374,6.194-5.214,6.194H2.831V5.806z" /></svg>
   }
}