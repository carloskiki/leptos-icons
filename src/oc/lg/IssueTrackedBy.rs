#[cfg(feature = "OcLgIssueTrackedBy")]
use leptos::*;
#[cfg(feature = "OcLgIssueTrackedBy")]
///This icon requires the feature `OcLgIssueTrackedBy` to be enabled.
#[component]
pub fn IssueTrackedBy(
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
        "M12 2.5a9.5 9.5 0 1 0 0 19 .75.75 0 0 1 0 1.5C5.925 23 1 18.075 1 12S5.925 1 12 1s11 4.925 11 11a.75.75 0 0 1-1.5 0A9.5 9.5 0 0 0 12 2.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m13.759 17.48 3.728 3.314a.308.308 0 0 0 .513-.23V18h4.25a.75.75 0 0 0 0-1.5H18v-2.564a.308.308 0 0 0-.513-.23L13.76 17.02a.308.308 0 0 0 0 .46ZM12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"
        /> < title > { title } < / title > < / svg >
    }
}
