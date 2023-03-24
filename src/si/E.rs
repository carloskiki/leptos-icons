#[cfg(feature = "SiE")]
use leptos::*;
#[cfg(feature = "SiE")]
///This icon requires the feature `SiE` to be enabled.
#[component]
pub fn E(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.13812.05332C9.6246-.2791 6.0635.95029 3.49234 3.53409.0641 6.97916-.94746 12.17083.93662 16.65094c1.88408 4.48013 6.30246 7.38884 11.16248 7.3483a2.27493 2.27493 0 10-.03759-4.54888c-3.02833.02526-5.75722-1.77058-6.93123-4.56223-1.174-2.79164-.54918-5.99816 1.58702-8.14484 2.13619-2.14669 5.339-2.78735 8.13635-1.62703 2.07161.85929 3.59079 2.57925 4.24094 4.64104H12.0506a2.27505 2.27505 0 100 4.5501h9.67488a2.27493 2.27493 0 002.27445-2.26717c.0167-4.86014-2.91357-9.264-7.40286-11.12612-1.12232-.46552-2.28778-.74998-3.45895-.8608Z"
        /> < title > { title } < / title > < / svg >
    }
}
