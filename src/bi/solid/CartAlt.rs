#[cfg(feature = "BiSolidCartAlt")]
use leptos::*;
#[cfg(feature = "BiSolidCartAlt")]
///This icon requires the feature `BiSolidCartAlt` to be enabled.
#[component]
pub fn CartAlt(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21 4H2v2h2.3l3.521 9.683A2.004 2.004 0 0 0 9.7 17H18v-2H9.7l-.728-2H18c.4 0 .762-.238.919-.606l3-7A.998.998 0 0 0 21 4z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "10.5" cy = "19.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "16.5" cy = "19.5" r = "1.5"
        /> < title > { title } < / title > < / svg >
    }
}
