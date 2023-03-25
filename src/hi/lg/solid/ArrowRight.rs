#[cfg(feature = "HiLgSolidArrowRight")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowRight")]
///This icon requires the feature `HiLgSolidArrowRight` to be enabled.
#[component]
pub fn ArrowRight(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.9697 3.96967C13.2626 3.67678 13.7374 3.67678 14.0303 3.96967L21.5303 11.4697C21.671 11.6103 21.75 11.8011 21.75 12C21.75 12.1989 21.671 12.3897 21.5303 12.5303L14.0303 20.0303C13.7374 20.3232 13.2626 20.3232 12.9697 20.0303C12.6768 19.7374 12.6768 19.2626 12.9697 18.9697L19.1893 12.75H3C2.58579 12.75 2.25 12.4142 2.25 12C2.25 11.5858 2.58579 11.25 3 11.25H19.1893L12.9697 5.03033C12.6768 4.73744 12.6768 4.26256 12.9697 3.96967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
