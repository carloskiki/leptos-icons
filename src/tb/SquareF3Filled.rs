#[cfg(feature = "TbSquareF3Filled")]
use leptos::*;
#[cfg(feature = "TbSquareF3Filled")]
///This icon requires the feature `TbSquareF3Filled` to be enabled.
#[component]
pub fn SquareF3Filled(
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
        "icon icon-tabler icon-tabler-square-f3-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.333 3a3.667 3.667 0 0 1 3.667 3.667v10.666a3.667 3.667 0 0 1 -3.667 3.667h-10.666a3.667 3.667 0 0 1 -3.667 -3.667v-10.666a3.667 3.667 0 0 1 3.667 -3.667zm-2.833 5h-1l-.144 .007a1.5 1.5 0 0 0 -1.356 1.493a1 1 0 0 0 1 1l.117 -.007a1 1 0 0 0 .727 -.457l.02 -.036h.636l.09 .008a.5 .5 0 0 1 0 .984l-.09 .008h-.5l-.133 .007c-1.156 .124 -1.156 1.862 0 1.986l.133 .007h.5l.09 .008a.5 .5 0 0 1 .41 .492l-.008 .09a.5 .5 0 0 1 -.492 .41h-.635l-.02 -.036a1 1 0 0 0 -1.845 .536a1.5 1.5 0 0 0 1.5 1.5h1l.164 -.005a2.5 2.5 0 0 0 2.336 -2.495l-.005 -.164a2.487 2.487 0 0 0 -.477 -1.312l-.019 -.024l.126 -.183a2.5 2.5 0 0 0 -2.125 -3.817zm-4.5 0h-2l-.117 .007a1 1 0 0 0 -.883 .993v6l.007 .117a1 1 0 0 0 .993 .883l.117 -.007a1 1 0 0 0 .883 -.993v-2h1l.117 -.007a1 1 0 0 0 -.117 -1.993h-1v-1h1l.117 -.007a1 1 0 0 0 -.117 -1.993z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
