#[cfg(feature = "SiBuffer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBuffer")]
/// *This icon requires the feature* `SiBuffer` *to be enabled*.
#[component]
pub fn Buffer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.371 5.476L11.943 0l10.686 5.476-10.686 5.495zm3.36 4.81l7.212 3.547 7.288-3.547 3.398 1.655-10.686 5.202L1.371 11.94zm0 6.171l7.212 3.911 7.288-3.91 3.398 1.815L11.943 24 1.371 18.273z" /></svg>
   }
}