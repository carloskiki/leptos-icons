#[cfg(feature = "HiMdSolidArrowDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowDown")]
///This icon requires the feature `HiMdSolidArrowDown` to be enabled.
#[component]
pub fn ArrowDown(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10 3C10.4142 3 10.75 3.33579 10.75 3.75L10.75 14.3879L14.7094 10.2302C14.9965 9.93159 15.4713 9.92228 15.7698 10.2094C16.0684 10.4965 16.0777 10.9713 15.7906 11.2698L10.5406 16.7698C10.3992 16.9169 10.204 17 10 17C9.79599 17 9.60078 16.9169 9.45938 16.7698L4.20938 11.2698C3.92228 10.9713 3.93159 10.4965 4.23017 10.2094C4.52875 9.92228 5.00353 9.93159 5.29063 10.2302L9.25 14.3879L9.25 3.75C9.25 3.33579 9.58579 3 10 3Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
