#[cfg(feature = "CgCornerDoubleRightUp")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleRightUp")]
///This icon requires the feature `CgCornerDoubleRightUp` to be enabled.
#[component]
pub fn CornerDoubleRightUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.2163 9.25045L17.6305 7.83624L13.3879 3.59363L9.14526 7.83627L10.5595 9.25048L13.3879 6.42205L16.2163 9.25045Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5595 13.4932L9.14526 12.0789L13.3879 7.83628L17.6305 12.0789L16.2163 13.4931L14.3695 11.6463V16.4063C14.3695 18.6155 12.5786 20.4063 10.3695 20.4063H6.36951V18.4063H10.3695C11.4741 18.4063 12.3695 17.5109 12.3695 16.4063V11.6831L10.5595 13.4932Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
