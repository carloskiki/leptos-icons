#[cfg(feature = "BiRegularHourglass")]
use leptos::*;
#[cfg(feature = "BiRegularHourglass")]
///This icon requires the feature `BiRegularHourglass` to be enabled.
#[component]
pub fn Hourglass(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15.566 11.021A7.016 7.016 0 0 0 19 5V4h1V2H4v2h1v1a7.016 7.016 0 0 0 3.434 6.021c.354.208.566.545.566.9v.158c0 .354-.212.69-.566.9A7.016 7.016 0 0 0 5 19v1H4v2h16v-2h-1v-1a7.014 7.014 0 0 0-3.433-6.02c-.355-.21-.567-.547-.567-.901v-.158c0-.355.212-.692.566-.9zm-1.015 3.681A5.008 5.008 0 0 1 17 19v1H7v-1a5.01 5.01 0 0 1 2.45-4.299c.971-.573 1.55-1.554 1.55-2.622v-.158c0-1.069-.58-2.051-1.551-2.623A5.008 5.008 0 0 1 7 5V4h10v1c0 1.76-.938 3.406-2.449 4.298C13.58 9.87 13 10.852 13 11.921v.158c0 1.068.579 2.049 1.551 2.623z"
        /> < title > { title } < / title > < / svg >
    }
}
