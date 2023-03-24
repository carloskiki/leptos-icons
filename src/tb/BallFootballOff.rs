#[cfg(feature = "TbBallFootballOff")]
use leptos::*;
#[cfg(feature = "TbBallFootballOff")]
///This icon requires the feature `TbBallFootballOff` to be enabled.
#[component]
pub fn BallFootballOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-ball-football-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.041 16.046a9 9 0 0 0 -12.084 -12.09m-2.323 1.683a9 9 0 0 0 12.726 12.73" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 7l4.755 3.455l-.566 1.743l-.98 3.014l-.209 .788h-6l-1.755 -5.545l1.86 -1.351l2.313 -1.681z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 7v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 16l2.5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.755 10.455l3.745 -1.455" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.061 16.045l-2.561 2.955" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.245 10.455l-3.745 -1.455" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
