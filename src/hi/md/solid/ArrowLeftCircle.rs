#[cfg(feature = "HiMdSolidArrowLeftCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowLeftCircle")]
///This icon requires the feature `HiMdSolidArrowLeftCircle` to be enabled.
#[component]
pub fn ArrowLeftCircle(
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
        "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" clip -
        path = "url(#clip0_9_2121)" >< path fill - rule = "evenodd" clip - rule =
        "evenodd" d =
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.25 10.75C13.6642 10.75 14 10.4142 14 10C14 9.58579 13.6642 9.25 13.25 9.25H8.6599L10.7603 7.29959C11.0639 7.01774 11.0814 6.54319 10.7996 6.23966C10.5177 5.93613 10.0432 5.91855 9.73966 6.2004L6.23966 9.4504C6.08684 9.59231 6 9.79145 6 10C6 10.2086 6.08684 10.4077 6.23966 10.5496L9.73966 13.7996C10.0432 14.0814 10.5177 14.0639 10.7996 13.7603C11.0814 13.4568 11.0639 12.9823 10.7603 12.7004L8.6599 10.75H13.25Z"
        fill = "#0F172A" /></ g >< defs xmlns = "http://www.w3.org/2000/svg" >< clipPath
        id = "clip0_9_2121" >< rect width = "20" height = "20" fill = "white" /></
        clipPath ></ defs > < title > { title } < / title > < / svg >
    }
}
