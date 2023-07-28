use cfg_if::cfg_if;
mod icons;

// boilerplate to run in different modes
cfg_if! {
    // server-only stuff
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use actix_files::{Files};
        use actix_web::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use icons::Icons;

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            //crate::icons::register_server_functions();

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            // when not using cargo-leptos None must be replaced with Some("Cargo.toml")
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;
            let routes = generate_route_list(|| view! { <Icons/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;

                App::new()
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), || view! { <Icons/> })
                    .service(Files::new("/", site_root))
                    //.wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
    }
    else {
        pub fn main() {}
    }
}
