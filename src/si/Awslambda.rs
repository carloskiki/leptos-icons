#[cfg(feature = "SiAwslambda")]
use leptos::*;
#[cfg(feature = "SiAwslambda")]
///This icon requires the feature `SiAwslambda` to be enabled.
#[component]
pub fn Awslambda(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M4.9855 0c-.2941.0031-.5335.2466-.534.5482L4.446 5.456c0 .1451.06.2835.159.3891a.5322.5322 0 0 0 .3806.1562h3.4282l8.197 17.6805a.5365.5365 0 0 0 .4885.3181h5.811c.2969 0 .5426-.2448.5426-.5482V18.544c0-.3035-.2392-.5482-.5425-.5482h-2.0138L12.7394.3153C12.647.124 12.4564 0 12.2452 0h-7.254Zm.5397 1.0907h6.3678l8.16 17.6804a.5365.5365 0 0 0 .4885.3181h1.8178v3.8173H17.437L9.2402 5.226a.536.536 0 0 0-.4885-.318H5.5223Zm2.0137 8.2366c-.2098.0011-.3937.1193-.4857.3096L.6002 23.2133a.5506.5506 0 0 0 .0313.5282.5334.5334 0 0 0 .4544.25h6.169a.5468.5468 0 0 0 .497-.3096l3.38-7.166a.5405.5405 0 0 0-.0029-.4686L8.036 9.637a.5468.5468 0 0 0-.4942-.3096Zm.0057 1.8036 2.488 5.1522-3.1214 6.6206H1.9465Z"
        /> < title > { title } < / title > < / svg >
    }
}
