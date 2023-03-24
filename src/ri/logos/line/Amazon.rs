#[cfg(feature = "RiLogosLineAmazon")]
use leptos::*;
#[cfg(feature = "RiLogosLineAmazon")]
///This icon requires the feature `RiLogosLineAmazon` to be enabled.
#[component]
pub fn Amazon(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M15.625 14.62c-1.107 1.619-2.728 2.384-4.625 2.384-2.304 0-4.276-1.773-3.993-4.124.315-2.608 2.34-3.73 5.708-4.143.601-.073.85-.094 2.147-.19l.138-.01v-.215C15 6.526 13.932 5.3 12.5 5.3c-1.437 0-2.44.747-3.055 2.526l-1.89-.652C8.442 4.604 10.193 3.3 12.5 3.3c2.603 0 4.5 2.178 4.5 5.022 0 2.649.163 4.756.483 5.557.356.892.486 1.117.884 1.613l-1.56 1.251c-.523-.652-.753-1.049-1.181-2.122v-.001zm5.632 5.925c-.271.2-.742.081-.529-.44.265-.648.547-1.408.262-1.752-.21-.255-.467-.382-1.027-.382-.46 0-.69.06-.995.08-.204.013-.293-.297-.091-.44a2.96 2.96 0 0 1 .87-.428c1.15-.344 2.505-.155 2.67.083.365.53-.199 2.569-1.16 3.28zm-1.182-1.084a7.555 7.555 0 0 1-.83.695c-2.122 1.616-4.87 2.46-7.258 2.46-3.843 0-7.28-1.793-9.888-4.795-.223-.23-.038-.566.223-.384 2.81 2.077 6.288 3.333 9.889 3.333 2.265 0 4.708-.537 7.035-1.693.162-.076.344-.18.503-.254.367-.21.69.306.326.638zm-5.065-8.92c-1.258.094-1.496.113-2.052.181-2.552.313-3.797 1.003-3.965 2.398-.126 1.043.81 1.884 2.007 1.884 2.039 0 3.517-1.228 4.022-4.463h-.012z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
