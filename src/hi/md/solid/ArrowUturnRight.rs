#[cfg(feature = "HiMdSolidArrowUturnRight")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowUturnRight")]
///This icon requires the feature `HiMdSolidArrowUturnRight` to be enabled.
#[component]
pub fn ArrowUturnRight(
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
        "M12.2075 2.23214C11.9215 2.53177 11.9325 3.00651 12.2321 3.29252L16.3781 7.25H6.375C3.40647 7.25 1 9.65647 1 12.625C1 15.5935 3.40647 18 6.375 18H9.25C9.66421 18 10 17.6642 10 17.25C10 16.8358 9.66421 16.5 9.25 16.5H6.375C4.2349 16.5 2.5 14.7651 2.5 12.625C2.5 10.4849 4.2349 8.75 6.375 8.75H16.3781L12.2321 12.7075C11.9325 12.9935 11.9215 13.4682 12.2075 13.7679C12.4935 14.0675 12.9682 14.0785 13.2679 13.7925L18.7679 8.54252C18.9161 8.401 19 8.20496 19 8C19 7.79504 18.9161 7.59901 18.7679 7.45748L13.2679 2.20748C12.9682 1.92148 12.4935 1.93252 12.2075 2.23214Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
