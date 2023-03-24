#[cfg(feature = "TbSnowflakeOff")]
use leptos::*;
#[cfg(feature = "TbSnowflakeOff")]
///This icon requires the feature `TbSnowflakeOff` to be enabled.
#[component]
pub fn SnowflakeOff(
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
        "icon icon-tabler icon-tabler-snowflake-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 4l2 1l2 -1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M12 2v6m1.196 1.186l1.804 1.034" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M17.928 6.268l.134 2.232l1.866 1.232"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.66 7l-5.629 3.25l-.031 .75" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M19.928 14.268l-1.015 .67" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.212 14.226l-2.171 1.262" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 20l-2 -1l-2 1" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 22v-6.5l-3 -1.72" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.072 17.732l-.134 -2.232l-1.866 -1.232" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3.34 17l5.629 -3.25l-.01 -3.458" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M4.072 9.732l1.866 -1.232l.134 -2.232"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3.34 7l5.629 3.25l.802 -.466"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
