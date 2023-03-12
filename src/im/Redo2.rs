#[cfg(feature = "ImRedo2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImRedo2")]
/// *This icon requires the feature* `ImRedo2` *to be enabled*.
#[component]
pub fn Redo2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 3.881v-3.881l6 6-6 6v-3.966c-6.98-0.164-6.681 4.747-4.904 7.966-4.386-4.741-3.455-12.337 4.904-12.119z" /></svg>
   }
}