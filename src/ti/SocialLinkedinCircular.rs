#[cfg(feature = "TiSocialLinkedinCircular")]
use leptos::*;
#[cfg(feature = "TiSocialLinkedinCircular")]
///This icon requires the feature `TiSocialLinkedinCircular` to be enabled.
#[component]
pub fn SocialLinkedinCircular(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M10.033 15.3h-1.6v-5.199h1.6v5.199zm-.8-5.866c-.577 0-.866-.267-.866-.8 0-.223.082-.412.25-.567.166-.155.371-.233.616-.233.577 0 .866.268.866.801s-.288.799-.866.799zm6.734 5.866h-1.633v-2.9c0-.755-.268-1.133-.801-1.133-.422 0-.699.211-.834.633-.043.067-.066.201-.066.4v3h-1.633v-3.533c0-.8-.012-1.355-.033-1.666h1.4l.1.699c.367-.556.9-.833 1.633-.833.557 0 1.006.194 1.35.583.346.389.518.95.518 1.684v3.066zM12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
