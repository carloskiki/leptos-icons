#[cfg(feature = "HiLgSolidArrowSmallLeft")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowSmallLeft")]
///This icon requires the feature `HiLgSolidArrowSmallLeft` to be enabled.
#[component]
pub fn ArrowSmallLeft(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M20.25 12C20.25 12.4142 19.9142 12.75 19.5 12.75L6.31066 12.75L11.7803 18.2197C12.0732 18.5126 12.0732 18.9874 11.7803 19.2803C11.4874 19.5732 11.0126 19.5732 10.7197 19.2803L3.96967 12.5303C3.67678 12.2374 3.67678 11.7626 3.96967 11.4697L10.7197 4.71967C11.0126 4.42678 11.4874 4.42678 11.7803 4.71967C12.0732 5.01256 12.0732 5.48744 11.7803 5.78033L6.31066 11.25L19.5 11.25C19.9142 11.25 20.25 11.5858 20.25 12Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
