#[cfg(feature = "CgCornerDoubleDownRight")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleDownRight")]
///This icon requires the feature `CgCornerDoubleDownRight` to be enabled.
#[component]
pub fn CornerDoubleDownRight(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.6004 7.67915L7.63814 2.74194L2.70093 7.7042L4.11871 9.11483L6.52249 6.69886L6.5075 12.7348C6.50092 15.3857 8.64461 17.5401 11.2956 17.5467L17.224 17.5614L14.9855 19.8638L16.4195 21.258L21.299 16.239L16.2801 11.3595L14.8859 12.7934L17.3217 15.1616L11.3015 15.1467C9.97605 15.1434 8.9042 14.0662 8.9075 12.7407L8.92214 6.84077L11.1898 9.09694L12.6004 7.67915Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
