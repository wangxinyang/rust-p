# arc-swap-coding
arc-swapでプロジェクトの配置データをHot Deployすることができます。

- axum
- serde
- arc-swap



>  **Focus code**

1. Extension(state)

   もしlet state = ArcSwap::new(Arc::new(config.features));に書けば、Cloneトレイトに満たさないになります。

   だからlet state = Arc::new(ArcSwap::new(Arc::new(config.features)));に設定する。

```
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::load().await;
    let state = Arc::new(ArcSwap::new(Arc::new(config.features)));
    let app = Router::new()
        .route("/", get(get_handler))
        .route("/reload", post(reload_handler))
        .layer(Extension(state));

    let addr = config.network.into();
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```



Config修正前

```
---
network:
  addr: 127.0.0.1
  port: 3200
features:
  font_size: 200
  font_color: white

```

![結果](https://s1.imagehub.cc/images/2022/10/14/3.png)

実行中Configファイルを修正する

```
---
network:
  addr: 127.0.0.1
  port: 3200
features:
  font_size: 300
  font_color: red

```

![reload](https://s1.imagehub.cc/images/2022/10/14/4.png)

![config](https://s1.imagehub.cc/images/2022/10/14/5.png)

