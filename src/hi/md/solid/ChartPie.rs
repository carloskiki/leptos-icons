#[cfg(feature = "HiMdSolidChartPie")]
use leptos::*;
#[cfg(feature = "HiMdSolidChartPie")]
///This icon requires the feature `HiMdSolidChartPie` to be enabled.
#[component]
pub fn ChartPie(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 8.99979C11.4477 8.99979 11 8.55207 11 7.99979V2.99979C11 2.4475 11.4503 1.99229 11.997 2.07025C15.0651 2.50777 17.492 4.93467 17.9295 8.00276C18.0075 8.54951 17.5523 8.99979 17 8.99979H12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.00297 4.07025C8.54972 3.99229 9 4.4475 9 4.99979V9.99979C9 10.5521 9.44772 10.9998 10 10.9998H15C15.5523 10.9998 16.0075 11.4501 15.9295 11.9968C15.4456 15.3906 12.5275 17.9998 9 17.9998C5.13401 17.9998 2 14.8658 2 10.9998C2 7.47229 4.60923 4.5542 8.00297 4.07025Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
