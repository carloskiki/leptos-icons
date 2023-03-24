#[cfg(feature = "BsCalendar3WeekFill")]
use leptos::*;
#[cfg(feature = "BsCalendar3WeekFill")]
///This icon requires the feature `BsCalendar3WeekFill` to be enabled.
#[component]
pub fn Calendar3WeekFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-calendar3-week-fill" viewBox = "0 0 16 16" width = { size.clone()
        } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" d =
        "M2 0a2 2 0 0 0-2 2h16a2 2 0 0 0-2-2H2zM0 14V3h16v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm12-8a1 1 0 1 0 2 0 1 1 0 0 0-2 0zM5 9a1 1 0 1 0 2 0 1 1 0 0 0-2 0zm5-2a1 1 0 1 1 0-2 1 1 0 0 1 0 2zM2 9a1 1 0 1 0 2 0 1 1 0 0 0-2 0z"
        /> < title > { title } < / title > < / svg >
    }
}
