#[cfg(feature = "TbAlertOctagonFilled")]
use leptos::*;
#[cfg(feature = "TbAlertOctagonFilled")]
///This icon requires the feature `TbAlertOctagonFilled` to be enabled.
#[component]
pub fn AlertOctagonFilled(
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
        "icon icon-tabler icon-tabler-alert-octagon-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.3 2c.5 0 .914 .16 1.274 .47l.133 .123l4.7 4.7c.348 .348 .546 .745 .586 1.224l.007 .183v6.6c0 .5 -.16 .914 -.47 1.274l-.123 .133l-4.7 4.7c-.348 .348 -.745 .546 -1.224 .586l-.183 .007h-6.6c-.5 0 -.914 -.16 -1.274 -.47l-.133 -.123l-4.7 -4.7c-.348 -.348 -.546 -.745 -.586 -1.224l-.007 -.183v-6.6c0 -.5 .16 -.914 .47 -1.274l.123 -.133l4.7 -4.7c.348 -.348 .745 -.546 1.224 -.586l.183 -.007h6.6zm-3.29 13l-.127 .007a1 1 0 0 0 0 1.986l.117 .007l.127 -.007a1 1 0 0 0 0 -1.986l-.117 -.007zm-.01 -8a1 1 0 0 0 -.993 .883l-.007 .117v4l.007 .117a1 1 0 0 0 1.986 0l.007 -.117v-4l-.007 -.117a1 1 0 0 0 -.993 -.883z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
