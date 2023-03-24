#[cfg(feature = "HiMdSolidLockClosed")]
use leptos::*;
#[cfg(feature = "HiMdSolidLockClosed")]
///This icon requires the feature `HiMdSolidLockClosed` to be enabled.
#[component]
pub fn LockClosed(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M10 1C7.51472 1 5.5 3.01472 5.5 5.5V9H5C3.89543 9 3 9.89543 3 11V17C3 18.1046 3.89543 19 5 19H15C16.1046 19 17 18.1046 17 17V11C17 9.89543 16.1046 9 15 9H14.5V5.5C14.5 3.01472 12.4853 1 10 1ZM13 9V5.5C13 3.84315 11.6569 2.5 10 2.5C8.34315 2.5 7 3.84315 7 5.5V9H13Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
