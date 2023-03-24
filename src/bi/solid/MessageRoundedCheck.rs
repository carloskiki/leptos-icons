#[cfg(feature = "BiSolidMessageRoundedCheck")]
use leptos::*;
#[cfg(feature = "BiSolidMessageRoundedCheck")]
///This icon requires the feature `BiSolidMessageRoundedCheck` to be enabled.
#[component]
pub fn MessageRoundedCheck(
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
        "M12 2C6.486 2 2 5.589 2 10c0 2.907 1.897 5.515 5 6.934V22l5.34-4.005C17.697 17.853 22 14.32 22 10c0-4.411-4.486-8-10-8zm-1 12.414-3.707-3.707 1.414-1.414L11 11.586l4.793-4.793 1.414 1.414L11 14.414z"
        /> < title > { title } < / title > < / svg >
    }
}
