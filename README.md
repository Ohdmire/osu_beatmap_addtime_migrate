# osu_beatmap_addtime_migrate
快速把stable里添加谱面的日期写入到lazer

## 使用方法(面向普通用户)
1. 到Release下载对应操作系统的最新版本zip解压
2. 运行`osu-beatmap-addtime-migrate.exe`生成`beatmap-hash-time.txt`
3. 运行`OsuRealmWriter.exe`将`beatmap-hash-time.txt`写入到lazer

### 提示
如果游戏不在默认安装位置请在运行完`osu-beatmap-addtime-migrate.exe`后手动修改`config.toml`文件

## 手动编译(面向开发者)
确保安装rust和dotnet sdk

- `cargo build`

- `dotnet build`

## 注意事项
请不要在lazer运行时运行此程序，否则会导致谱面信息写入失败


# 使用项目
https://github.com/PinNaCode/CollectionDowngrader