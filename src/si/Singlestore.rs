#[cfg(feature = "SiSinglestore")]
use leptos::*;
#[cfg(feature = "SiSinglestore")]
///This icon requires the feature `SiSinglestore` to be enabled.
#[component]
pub fn Singlestore(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.029 5.063c-.914-1.916-2.8-3.432-5.114-4.033C11.4.887 10.858.83 10.258.8c-.886 0-1.743.114-2.629.343-2.2.658-3.742 1.945-4.657 2.947C1.801 5.435 1.03 6.837.572 8.238c0 .029-.028.057-.028.115C.515 8.467.4 8.81.4 8.896c-.029.057-.029.143-.057.2l-.086.344c0 .028 0 .057-.028.086-.743 3.69.49 7.001 1.234 8.231.185.308.338.564.49.8C.27 9.403 5.116 5.035 10.63 4.92c2.886-.057 5.771 1.116 7.171 3.461-.086-1.287-.171-2.002-.771-3.318zM12.543 0c2.572.715 4.914 2.517 5.886 4.72 1.485 3.575 1.143 8.095-.486 10.784-1.371 2.203-3.485 3.375-5.914 3.347-3.771-.029-6.828-3.032-6.857-6.808 0-3.776 2.971-6.894 6.857-6.894.629 0 1.535.087 2.563.516 0 0-.739-.438-2.638-.732C6.497 4.218.058 8.353 1.544 17.878c2.057 3.662 6 6.15 10.485 6.122 6.6-.029 12-5.435 11.97-12.072C24 5.578 18.83.172 12.544 0z"
        /> < title > { title } < / title > < / svg >
    }
}
