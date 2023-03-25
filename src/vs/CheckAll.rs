#[cfg(feature = "VsCheckAll")]
use leptos::*;
#[cfg(feature = "VsCheckAll")]
///This icon requires the feature `VsCheckAll` to be enabled.
#[component]
pub fn CheckAll(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15.62 3.596L7.815 12.81l-.728-.033L4 8.382l.754-.53 2.744 3.907L14.917 3l.703.596z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d =
        "M7.234 8.774l4.386-5.178L10.917 3l-4.23 4.994.547.78zm-1.55.403l.548.78-.547-.78zm-1.617 1.91l.547.78-.799.943-.728-.033L0 8.382l.754-.53 2.744 3.907.57-.672z"
        /> < title > { title } < / title > < / svg >
    }
}
