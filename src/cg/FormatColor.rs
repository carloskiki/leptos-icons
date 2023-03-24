#[cfg(feature = "CgFormatColor")]
use leptos::*;
#[cfg(feature = "CgFormatColor")]
///This icon requires the feature `CgFormatColor` to be enabled.
#[component]
pub fn FormatColor(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.9479 3.20946C12.7721 2.83236 12.391 2.61645 11.9997 2.63269C11.6086 2.61666 11.2278 2.83255 11.0521 3.20948L5.1354 15.8978C4.90199 16.3983 5.11855 16.9933 5.61909 17.2267C6.11963 17.4601 6.71461 17.2436 6.94801 16.743L8.39869 13.632H15.6013L17.052 16.743C17.2854 17.2435 17.8804 17.4601 18.3809 17.2267C18.8814 16.9933 19.098 16.3983 18.8646 15.8978L12.9479 3.20946ZM14.6687 11.632L12 5.909L9.33131 11.632H14.6687Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 19.3682C5.44772 19.3682 5 19.816 5 20.3682C5 20.9205 5.44772 21.3682 6 21.3682H18C18.5523 21.3682 19 20.9205 19 20.3682C19 19.816 18.5523 19.3682 18 19.3682H6Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
