#[cfg(feature = "TbCircleArrowDownRightFilled")]
use leptos::*;
#[cfg(feature = "TbCircleArrowDownRightFilled")]
///This icon requires the feature `TbCircleArrowDownRightFilled` to be enabled.
#[component]
pub fn CircleArrowDownRightFilled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-circle-arrow-down-right-filled" width = "24" height
        = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 3.34a10 10 0 1 1 -14.995 8.984l-.005 -.324l.005 -.324a10 10 0 0 1 14.995 -8.336zm-2 4.66l-.117 .007a1 1 0 0 0 -.883 .993v3.585l-4.293 -4.292l-.094 -.083a1 1 0 0 0 -1.32 1.497l4.292 4.293h-3.585l-.117 .007a1 1 0 0 0 .117 1.993l6.034 .001a.998 .998 0 0 0 .186 -.025l.053 -.014l.066 -.02l.13 -.059l.093 -.055a.98 .98 0 0 0 .438 -.828v-6l-.007 -.117a1 1 0 0 0 -.993 -.883z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
