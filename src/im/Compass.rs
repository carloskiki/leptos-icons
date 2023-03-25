#[cfg(feature = "ImCompass")]
use leptos::*;
#[cfg(feature = "ImCompass")]
///This icon requires the feature `ImCompass` to be enabled.
#[component]
pub fn Compass(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8.5 16c-0.036 0-0.072-0.004-0.108-0.012-0.229-0.051-0.392-0.254-0.392-0.488v-7.5h-7.5c-0.234 0-0.437-0.163-0.488-0.392s0.064-0.462 0.277-0.561l15-7c0.191-0.089 0.416-0.049 0.565 0.1s0.188 0.374 0.1 0.565l-7 15c-0.083 0.179-0.262 0.289-0.453 0.289zM2.754 7h5.746c0.276 0 0.5 0.224 0.5 0.5v5.746l5.465-11.712-11.712 5.465z"
        /> < title > { title } < / title > < / svg >
    }
}
