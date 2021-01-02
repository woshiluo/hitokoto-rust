# hitokoto-rust

> 一个使用 Rust 写的 Hitokoto server

使用方式

```
hitokoto-rust 0.1

hitokoto-rust [OPTIONS] --listen <IP:PORT> --path <PATH> -t <UNSIGNED INT>

-d <LOG LEVEL>
-l, --listen <IP:PORT>    The ip and port will be listend.
-p, --path <PATH>         Hitokoto sentences path.
-t <UNSIGNED INT>         Threads number
```

网页端 API

```
http://ip:port/$cat_id/$hitokot_id
```
返回 `$cat_id` 下第 `$hitokot_id` 个一言

如果 `$cat_id` 为 0，则随机一个  
如果 `$hitokoto_id` 为 0，则随机一个


## 致谢

<https://github.com/hitokoto-osc/sentences-bundle/>
