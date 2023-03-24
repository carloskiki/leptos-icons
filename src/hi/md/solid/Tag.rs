#[cfg(feature = "HiMdSolidTag")]
use leptos::*;
#[cfg(feature = "HiMdSolidTag")]
///This icon requires the feature `HiMdSolidTag` to be enabled.
#[component]
pub fn Tag(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5.5 3C4.11929 3 3 4.11929 3 5.5V8.37868C3 9.04172 3.26339 9.67761 3.73223 10.1464L10.2322 16.6464C11.2085 17.6228 12.7915 17.6228 13.7678 16.6464L16.6464 13.7678C17.6228 12.7915 17.6228 11.2085 16.6464 10.2322L10.1464 3.73223C9.67761 3.26339 9.04172 3 8.37868 3H5.5ZM6 7C6.55228 7 7 6.55228 7 6C7 5.44772 6.55228 5 6 5C5.44772 5 5 5.44772 5 6C5 6.55228 5.44772 7 6 7Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
