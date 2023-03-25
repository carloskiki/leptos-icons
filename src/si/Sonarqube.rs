#[cfg(feature = "SiSonarqube")]
use leptos::*;
#[cfg(feature = "SiSonarqube")]
///This icon requires the feature `SiSonarqube` to be enabled.
#[component]
pub fn Sonarqube(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.685.386l-.465.766c3.477 2.112 6.305 5.27 7.966 8.89L24 9.67C22.266 5.887 19.313 2.59 15.685.386zM8.462.91l-.305 1.075c6.89 1.976 12.384 7.64 13.997 14.421l1.085-.258C21.535 8.977 15.735 2.997 8.462.909zM0 2.667v1.342c10.963 0 19.883 8.795 19.883 19.605h1.342c0-11.55-9.522-20.947-21.225-20.947z"
        /> < title > { title } < / title > < / svg >
    }
}
