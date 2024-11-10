# osu_beatmap_addtime_migrate
快速把osu里添加谱面的日期写入到lazer

## 使用方法
1. 运行`main.exe`生成`beatmap-hash-time.txt`
2. 运行`OsuRealmWriter.exe`将`beatmap-hash-time.txt`写入到lazer

## 编译
确保安装rust和dotnet sdk

- `cargo build`

- `dotnet build`

## 注意事项
请不要在lazer运行时运行此程序，否则会导致谱面信息写入失败


# 使用项目
https://github.com/PinNaCode/CollectionDowngrader