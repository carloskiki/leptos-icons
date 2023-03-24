#[cfg(feature = "BiRegularPaperclip")]
use leptos::*;
#[cfg(feature = "BiRegularPaperclip")]
///This icon requires the feature `BiRegularPaperclip` to be enabled.
#[component]
pub fn Paperclip(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M17.004 5H9c-1.838 0-3.586.737-4.924 2.076C2.737 8.415 2 10.163 2 12c0 1.838.737 3.586 2.076 4.924C5.414 18.263 7.162 19 9 19h8v-2H9c-1.303 0-2.55-.529-3.51-1.49C4.529 14.55 4 13.303 4 12c0-1.302.529-2.549 1.49-3.51C6.45 7.529 7.697 7 9 7h8V6l.001 1h.003c.79 0 1.539.314 2.109.886.571.571.886 1.322.887 2.116a2.966 2.966 0 0 1-.884 2.11A2.988 2.988 0 0 1 17 13H9a.99.99 0 0 1-.698-.3A.991.991 0 0 1 8 12c0-.252.11-.507.301-.698A.987.987 0 0 1 9 11h8V9H9c-.79 0-1.541.315-2.114.889C6.314 10.461 6 11.211 6 12s.314 1.54.888 2.114A2.974 2.974 0 0 0 9 15h8.001a4.97 4.97 0 0 0 3.528-1.473 4.967 4.967 0 0 0-.001-7.055A4.95 4.95 0 0 0 17.004 5z"
        /> < title > { title } < / title > < / svg >
    }
}
