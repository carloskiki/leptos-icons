#[cfg(feature = "BiSolidCaretUpCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCaretUpCircle")]
/// *This icon requires the feature* `BiSolidCaretUpCircle` *to be enabled*.
#[component]
pub fn CaretUpCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.485 2 2 6.486 2 12s4.485 10 10 10c5.514 0 10-4.486 10-10S17.514 2 12 2zM7 14l5-6 5 6H7z" /></svg>
   }
}