#[cfg(feature = "OcSmIssueTrackedBy")]
use leptos::*;
#[cfg(feature = "OcSmIssueTrackedBy")]
///This icon requires the feature `OcSmIssueTrackedBy` to be enabled.
#[component]
pub fn IssueTrackedBy(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5 8a6.5 6.5 0 0 1 13 0A.75.75 0 0 0 16 8a8 8 0 1 0-8 8 .75.75 0 0 0 0-1.5A6.5 6.5 0 0 1 1.5 8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 9.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3Zm3.573 5.823-2.896-2.896a.25.25 0 0 1 0-.354l2.896-2.896a.25.25 0 0 1 .427.177V11.5h3.25a.75.75 0 0 1 0 1.5H12v2.146a.25.25 0 0 1-.427.177Z"
        /> < title > { title } < / title > < / svg >
    }
}
