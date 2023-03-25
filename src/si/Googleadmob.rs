#[cfg(feature = "SiGoogleadmob")]
use leptos::*;
#[cfg(feature = "SiGoogleadmob")]
///This icon requires the feature `SiGoogleadmob` to be enabled.
#[component]
pub fn Googleadmob(
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
        "M11.46.033h-.052A11.993 11.993 0 0 0 0 11.922v.052c0 7.475 6.563 11.928 11.447 11.928h.17a3.086 3.086 0 0 0 3.125-3.047c0-1.693-1.433-2.917-3.152-2.917h-.039a6.016 6.016 0 0 1-5.508-6.368v-.052a6.016 6.016 0 0 1 5.573-5.509c1.719 0 3.125-1.237 3.125-2.917A3.086 3.086 0 0 0 11.604.02h-.143zm2.031.026a3.516 3.516 0 0 1 1.746 3.021 3.386 3.386 0 0 1-1.928 3.047c2.865.6 4.532 3.126 4.688 5.378v7.684a3.49 3.49 0 0 1 6.003.026v-7.736A12.046 12.046 0 0 0 13.491.045zm7.475 17.932a2.995 2.995 0 1 0 .04 0z"
        /> < title > { title } < / title > < / svg >
    }
}
