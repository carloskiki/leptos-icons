#[cfg(feature = "VsItalic")]
use leptos::*;
#[cfg(feature = "VsItalic")]
///This icon requires the feature `VsItalic` to be enabled.
#[component]
pub fn Italic(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.001 13.593l-.097.325H4l.123-.325c.492-.012.817-.053.976-.123.257-.1.448-.238.57-.413.194-.276.394-.768.599-1.477l2.074-7.19c.176-.597.263-1.048.263-1.353a.643.643 0 0 0-.114-.387.683.683 0 0 0-.351-.237c-.153-.059-.454-.088-.906-.088L7.34 2h4.605l-.096.325c-.375-.006-.654.035-.835.123a1.358 1.358 0 0 0-.607.501c-.134.217-.31.697-.527 1.442l-2.066 7.19c-.187.661-.28 1.083-.28 1.265 0 .146.034.272.105.378.076.1.193.178.351.237.164.053.501.097 1.011.132z"
        /> < title > { title } < / title > < / svg >
    }
}
