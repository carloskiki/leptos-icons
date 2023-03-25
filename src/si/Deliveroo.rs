#[cfg(feature = "SiDeliveroo")]
use leptos::*;
#[cfg(feature = "SiDeliveroo")]
///This icon requires the feature `SiDeliveroo` to be enabled.
#[component]
pub fn Deliveroo(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.861 0l-1.127 10.584L13.81 1.66 7.777 2.926l1.924 8.922-8.695 1.822 1.535 7.127L17.832 24l3.498-7.744L22.994.636 16.861 0zM11.39 13.61a.755.755 0 01.322.066c.208.093.56.29.63.592.103.434.004.799-.312 1.084v.002c-.315.284-.732.258-1.174.113-.441-.145-.637-.672-.47-1.309.124-.473.71-.544 1.004-.549zm4.142.548c.447-.012.832.186 1.05.543.217.357.107.75-.122 1.143h-.002c-.229.392-.83.445-1.422.16-.399-.193-.397-.684-.353-.983a.922.922 0 01.193-.447c.142-.177.381-.408.656-.416Z"
        /> < title > { title } < / title > < / svg >
    }
}
