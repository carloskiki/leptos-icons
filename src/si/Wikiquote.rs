#[cfg(feature = "SiWikiquote")]
use leptos::*;
#[cfg(feature = "SiWikiquote")]
///This icon requires the feature `SiWikiquote` to be enabled.
#[component]
pub fn Wikiquote(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M10.152 12a4.037 4.037 0 1 1-8.075 0 4.037 4.037 0 0 1 8.075 0zM17.292.822c-.286-.287-.581-.56-.885-.822l-1.528 1.527C17.872 4.036 19.778 7.8 19.778 12s-1.906 7.964-4.899 10.473L16.407 24c.304-.262.6-.535.886-.822A15.705 15.705 0 0 0 21.923 12c0-4.223-1.644-8.192-4.63-11.178zM13.508 2.9L12.03 4.377a9.642 9.642 0 0 1 0 15.246l1.477 1.477a11.712 11.712 0 0 0 0-18.2zm-2.735 2.735L9.349 7.057c1.61 1.057 2.675 2.878 2.675 4.943s-1.065 3.886-2.675 4.943l1.423 1.422A7.884 7.884 0 0 0 14.005 12a7.884 7.884 0 0 0-3.233-6.365z"
        /> < title > { title } < / title > < / svg >
    }
}
