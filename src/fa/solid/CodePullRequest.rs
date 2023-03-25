#[cfg(feature = "FaSolidCodePullRequest")]
use leptos::*;
#[cfg(feature = "FaSolidCodePullRequest")]
///This icon requires the feature `FaSolidCodePullRequest` to be enabled.
#[component]
pub fn CodePullRequest(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M305.8 2.1C314.4 5.9 320 14.5 320 24V64h16c70.7 0 128 57.3 128 128V358.7c28.3 12.3 48 40.5 48 73.3c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-32.8 19.7-61 48-73.3V192c0-35.3-28.7-64-64-64H320v40c0 9.5-5.6 18.1-14.2 21.9s-18.8 2.3-25.8-4.1l-80-72c-5.1-4.6-7.9-11-7.9-17.8s2.9-13.3 7.9-17.8l80-72c7-6.3 17.2-7.9 25.8-4.1zM104 80c0-13.3-10.7-24-24-24S56 66.7 56 80s10.7 24 24 24s24-10.7 24-24zm8 73.3V358.7c28.3 12.3 48 40.5 48 73.3c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-32.8 19.7-61 48-73.3V153.3C19.7 141 0 112.8 0 80C0 35.8 35.8 0 80 0s80 35.8 80 80c0 32.8-19.7 61-48 73.3zM104 432c0-13.3-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24s24-10.7 24-24zm328 24c13.3 0 24-10.7 24-24s-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24z"
        /> < title > { title } < / title > < / svg >
    }
}
