#[cfg(feature = "HiLgSolidArrowTrendingDown")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowTrendingDown")]
///This icon requires the feature `HiLgSolidArrowTrendingDown` to be enabled.
#[component]
pub fn ArrowTrendingDown(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1.71967 5.46967C2.01256 5.17678 2.48744 5.17678 2.78033 5.46967L9 11.6893L12.7558 7.9335C13.0214 7.66793 13.4425 7.63974 13.7411 7.86752C15.9037 9.51731 17.5581 11.8701 18.3164 14.7L18.6242 15.8488L20.9009 11.9056C21.108 11.5468 21.5667 11.4239 21.9254 11.631C22.2841 11.8381 22.407 12.2968 22.1999 12.6556L19.0179 18.1669C18.9185 18.3392 18.7546 18.4649 18.5625 18.5164C18.3704 18.5678 18.1657 18.5409 17.9934 18.4414L12.482 15.2595C12.1233 15.0523 12.0004 14.5937 12.2075 14.2349C12.4146 13.8762 12.8733 13.7533 13.232 13.9604L17.1753 16.2371L16.8675 15.0882C16.2588 12.8165 14.9977 10.8956 13.3392 9.47141L9.53033 13.2803C9.23744 13.5732 8.76256 13.5732 8.46967 13.2803L1.71967 6.53033C1.42678 6.23744 1.42678 5.76256 1.71967 5.46967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
