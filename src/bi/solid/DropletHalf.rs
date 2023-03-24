#[cfg(feature = "BiSolidDropletHalf")]
use leptos::*;
#[cfg(feature = "BiSolidDropletHalf")]
///This icon requires the feature `BiSolidDropletHalf` to be enabled.
#[component]
pub fn DropletHalf(
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
        "M12.578 2.184a1.004 1.004 0 0 0-1.156 0C11.119 2.398 4 7.513 4 13.75 4 18.53 7.364 22 12 22s8-3.468 8-8.246c0-6.241-7.119-11.356-7.422-11.57zM6 13.75c0-4.283 4.395-8.201 6-9.49V20c-3.533 0-6-2.57-6-6.25z"
        /> < title > { title } < / title > < / svg >
    }
}
