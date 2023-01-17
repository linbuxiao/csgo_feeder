# CSGO战绩订阅 _csgo_feeder_ 


> 一个用来查看 CSGO 战友实时战绩的工具

## 为什么需要这个工具
最开始，我的小舅子作为代练经常帮我打号上分，我想在不刷平台 APP 的情况下实时得知战绩，并一起进行讨论。

后来，战友们由于时间问题无法一起游戏，但是我想了解他们的上分进度，于是有了这个工具。

## 安装
### 本地编译
1. clone 此仓库，运行 `cargo build --release`。
2. 查看 `target/release/csgo_feeder`。

### Github Release
TODO

## 使用
1. 运行 csgo_feeder, 此时 `~/.config/csgo_feeder/default-config.toml` 会生成，这是一个初始化配置文件。
```toml
# 以下所有 steam_id 相关均为长 steam_id，具体可使用 https://www.steamidfinder.com/
# 查找 STEAMID64(Dec)
my_steam_id = '' # 你自己的 ID 
access_token = '' # wmapi access token，需要自行抓包
webhook_key = '' # 企微 bot webhook key，进行推送

[feed_map]
# 以下需要设置订阅的人，具体格式如下：
# 订阅 id = 昵称
```

2. 配置好后再次运行即可。

## TODO / NEED HELP
- [ ] 生成 Release 推送至 Github
- [ ] 支持更多推送模式
- [ ] vhs 生成运行 gif

