#[cfg(feature = "TbBallVolleyball")]
use leptos::*;
#[cfg(feature = "TbBallVolleyball")]
///This icon requires the feature `TbBallVolleyball` to be enabled.
#[component]
pub fn BallVolleyball(
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
        "icon icon-tabler icon-tabler-ball-volleyball" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12a8 8 0 0 0 8 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.5 13.5a12 12 0 0 0 8.5 6.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12a8 8 0 0 0 -7.464 4.928" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12.951 7.353a12 12 0 0 0 -9.88 4.111" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M12 12a8 8 0 0 0 -.536 -8.928" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15.549 15.147a12 12 0 0 0 1.38 -10.611" /> < title > { title } < / title > < /
        svg >
    }
}
