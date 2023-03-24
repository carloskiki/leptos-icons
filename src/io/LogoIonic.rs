#[cfg(feature = "IoLogoIonic")]
use leptos::*;
#[cfg(feature = "IoLogoIonic")]
///This icon requires the feature `IoLogoIonic` to be enabled.
#[component]
pub fn LogoIonic(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,153.9A102.1,102.1,0,1,0,358.1,256,102.23,102.23,0,0,0,256,153.9Z" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "402.59" cy = "116.45" r =
        "46.52" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M459.86,163.2l-1.95-4.28-3.11,3.52a70,70,0,0,1-28.06,19.32l-3,1.1,1.22,2.93A181.43,181.43,0,0,1,439,256c0,100.92-82.1,183-183,183S73,356.92,73,256,155.08,73,256,73a180.94,180.94,0,0,1,78.43,17.7L337.3,92l1.25-2.92A70.19,70.19,0,0,1,359.21,62l3.67-2.93L358.71,57A221.61,221.61,0,0,0,256,32C132.49,32,32,132.49,32,256S132.49,480,256,480,480,379.51,480,256A222.19,222.19,0,0,0,459.86,163.2Z"
        /> < title > { title } < / title > < / svg >
    }
}
