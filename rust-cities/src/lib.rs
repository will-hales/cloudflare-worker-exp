use worker::*;
use serde::{Deserialize, Serialize};

mod queue;

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let router = Router::new();
    router
    .post_async("/:country", |mut req, ctx| async move {
        let country = ctx.param("country").unwrap();

        #[derive(Deserialize, Serialize, Debug)]
        struct Country {
            city: String,
        }

        let city = req.json::<Country>().await?.city;

        console_log!("{:?}: {:?}", country, city);
        
        return match ctx.kv("cities")?.put(country, &city)?.execute().await {
            Ok(_) => Response::ok(city),
            Err(_) => Response::error("Bad request", 400),
        };
    })
    .get_async("/:country", |_req, ctx| async move {
        let country = ctx.param("country").unwrap();

        return match ctx.kv("cities")?.get(country).text().await? {
            Some(city) => Response::ok(city),
            None => Response::error("Country not found", 404),
        };
    })
    .post_async("/queue/:value", |_req, ctx| async move {
        let value = ctx.param("value").unwrap();
        queue::add(value);
        Response::ok(format!("Added {} to the queue", value))
    })
    .get_async("/queue", |_req, _ctx| async move {
        let queue = queue::get_queue();
        Response::ok(format!("Queue: {:?}", queue))
    })
    .run(_req, _env)
    .await
}