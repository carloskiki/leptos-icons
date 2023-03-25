#[cfg(feature = "FaBrandsGitlab")]
use leptos::*;
#[cfg(feature = "FaBrandsGitlab")]
///This icon requires the feature `FaBrandsGitlab` to be enabled.
#[component]
pub fn Gitlab(
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
        "M503.5 204.6L502.8 202.8L433.1 21.02C431.7 17.45 429.2 14.43 425.9 12.38C423.5 10.83 420.8 9.865 417.9 9.57C415 9.275 412.2 9.653 409.5 10.68C406.8 11.7 404.4 13.34 402.4 15.46C400.5 17.58 399.1 20.13 398.3 22.9L351.3 166.9H160.8L113.7 22.9C112.9 20.13 111.5 17.59 109.6 15.47C107.6 13.35 105.2 11.72 102.5 10.7C99.86 9.675 96.98 9.295 94.12 9.587C91.26 9.878 88.51 10.83 86.08 12.38C82.84 14.43 80.33 17.45 78.92 21.02L9.267 202.8L8.543 204.6C-1.484 230.8-2.72 259.6 5.023 286.6C12.77 313.5 29.07 337.3 51.47 354.2L51.74 354.4L52.33 354.8L158.3 434.3L210.9 474L242.9 498.2C246.6 500.1 251.2 502.5 255.9 502.5C260.6 502.5 265.2 500.1 268.9 498.2L300.9 474L353.5 434.3L460.2 354.4L460.5 354.1C482.9 337.2 499.2 313.5 506.1 286.6C514.7 259.6 513.5 230.8 503.5 204.6z"
        /> < title > { title } < / title > < / svg >
    }
}
