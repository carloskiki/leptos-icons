#[cfg(feature = "FaBrandsMixcloud")]
use leptos::*;
#[cfg(feature = "FaBrandsMixcloud")]
///This icon requires the feature `FaBrandsMixcloud` to be enabled.
#[component]
pub fn Mixcloud(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M212.98 346.566H179.789V195.114L185.973 173.47H175.262L137.127 346.566H76.1069L37.7323 173.47H27.276L33.1913 195.114V346.566H0V165H65.6506L102.248 338.096H110.747L147.329 165H212.98L212.98 346.566ZM544.459 283.589L458.434 345.655V307.534L531.329 255.776L458.434 204.017V165.896L544.459 228.231H553.721L640 165.896V204.017L566.866 255.776L640 307.549V345.655L553.721 283.589H544.459ZM430.157 272.311H248.113V239.255H430.157V272.311Z"
        /> < title > { title } < / title > < / svg >
    }
}
