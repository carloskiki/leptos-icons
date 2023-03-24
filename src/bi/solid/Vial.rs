#[cfg(feature = "BiSolidVial")]
use leptos::*;
#[cfg(feature = "BiSolidVial")]
///This icon requires the feature `BiSolidVial` to be enabled.
#[component]
pub fn Vial(
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
        "M7 22a4.965 4.965 0 0 0 3.535-1.465l9.193-9.193.707.708 1.414-1.414-8.485-8.486-1.414 1.414.708.707-9.193 9.193C2.521 14.408 2 15.664 2 17s.521 2.592 1.465 3.535A4.965 4.965 0 0 0 7 22zM18.314 9.928 15.242 13H6.758l7.314-7.314 4.242 4.242z"
        /> < title > { title } < / title > < / svg >
    }
}
