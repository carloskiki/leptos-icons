#[cfg(feature = "SiCommonworkflowlanguage")]
use leptos::*;
#[cfg(feature = "SiCommonworkflowlanguage")]
///This icon requires the feature `SiCommonworkflowlanguage` to be enabled.
#[component]
pub fn Commonworkflowlanguage(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.905 0L8.571 5.4l.037.037.096.096 3.586 3.395-2.24 2.252h-.01l-1.576 1.586 3.737 3.766-3.735 3.803.126.139v.012L12.052 24l1.608-1.64-1.98-2.034 3.737-3.79-1.608-1.642-.01.012-2.13-2.129 3.867-3.866-.017-.015.016-.016-3.641-3.524 3.64-3.694z"
        /> < title > { title } < / title > < / svg >
    }
}
