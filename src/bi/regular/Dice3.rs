#[cfg(feature = "BiRegularDice3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDice3")]
/// *This icon requires the feature* `BiRegularDice3` *to be enabled*.
#[component]
pub fn Dice3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM5 19V5h14l.002 14H5z" /><circle cx="8" cy="8" r="1.5" /><circle cx="12" cy="12" r="1.5" /><circle cx="16" cy="16" r="1.5" /></svg>
   }
}