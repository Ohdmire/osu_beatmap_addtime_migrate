using Realms;
using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using Tommy;

using CollectionDowngrader.LazerSchema;

class Program
{
    static string GetLazerDbPath()
    {
        const string configFile = "config.toml";
        if (!File.Exists(configFile))
        {
            throw new FileNotFoundException($"找不到配置文件: {configFile}");
        }

        using var reader = File.OpenText(configFile);
        var toml = TOML.Parse(reader);
        
        string lazerDir = toml["lazer_dir"];
        if (string.IsNullOrEmpty(lazerDir))
        {
            throw new Exception("配置文件中未找到 lazer_dir 设置");
        }

        string realmPath = Path.Combine(lazerDir, "client.realm");
        if (!File.Exists(realmPath))
        {
            throw new FileNotFoundException($"找不到 osu!lazer 数据库文件: {realmPath}\n请检查 config.toml 中的 lazer_dir 设置是否正确");
        }
        return realmPath;
    }

    static void CleanupRealmLockFiles(string realmPath)
    {
        var lockFiles = new[] {
            realmPath + ".lock",
            realmPath + ".note",
            realmPath + ".management"
        };

        foreach (var lockFile in lockFiles)
        {
            try
            {
                if (File.Exists(lockFile))
                {
                    File.Delete(lockFile);
                    Console.WriteLine($"已删除锁定文件: {lockFile}");
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"警告: 无法删除锁定文件 {lockFile}: {ex.Message}");
            }
        }
    }

    static void Main(string[] args)
    {
        string backupDb = string.Empty;
        
        try
        {
            const int LazerSchemaVersion = 42;
            const string hashTimeFile = "beatmap-hash-time.txt";
            
            string originalDb = GetLazerDbPath();
            backupDb = originalDb + ".bak";

            // 在打开数据库之前清理锁定文件
            CleanupRealmLockFiles(originalDb);

            // 创建备份
            if (File.Exists(originalDb))
            {
                File.Copy(originalDb, backupDb, true);
                Console.WriteLine($"已创建数据库备份: {backupDb}");
            }

            // 读取 beatmap-hash-time.txt 文件到字典中
            var hashToDateDict = new Dictionary<string, DateTime>();
            foreach (var line in File.ReadAllLines(hashTimeFile))
            {
                var parts = line.Split(',');
                if (parts.Length == 2)
                {
                    string hash = parts[0].Trim();
                    if (DateTime.TryParse(parts[1].Trim(), out DateTime date))
                    {
                        hashToDateDict[hash] = date;
                    }
                }
            }

            Console.WriteLine($"成功读取了 {hashToDateDict.Count} 条哈希时间记录");

            var config = new RealmConfiguration(originalDb)
            {
                SchemaVersion = LazerSchemaVersion,
            };
            config.Schema = new[] {
                typeof(Beatmap),
                typeof(BeatmapCollection),
                typeof(BeatmapDifficulty),
                typeof(BeatmapMetadata),
                typeof(BeatmapSet),
                typeof(BeatmapUserSettings),
                typeof(RealmFile),
                typeof(RealmNamedFileUsage),
                typeof(RealmUser),
                typeof(Ruleset),
                typeof(ModPreset)
            };

            using var realm = Realm.GetInstance(config);
            
            var beatmapSets = realm.All<BeatmapSet>();
            int updateCount = 0;

            realm.Write(() =>
            {
                foreach (var beatmapSet in beatmapSets)
                {
                    // 收集这个谱面集所有谱面的时间
                    var times = new List<DateTime>();
                    foreach (var beatmap in beatmapSet.Beatmaps)
                    {
                        if (hashToDateDict.TryGetValue(beatmap.MD5Hash, out DateTime addTime))
                        {
                            times.Add(addTime);
                        }
                    }

                    // 如果找到了时间记录，使用最新的时间
                    if (times.Any())
                    {
                        var latestTime = times.Max();
                        beatmapSet.DateAdded = new DateTimeOffset(latestTime);
                        updateCount++;
                        if (updateCount % 100 == 0)
                        {
                            Console.WriteLine($"已更新 {updateCount} 个 beatmapSet");
                        }
                    }
                }
            });

            Console.WriteLine($"总共更新了 {updateCount} 个 beatmapSet");
            Console.WriteLine($"如果需要恢复，可以使用备份文件: {backupDb}");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"发生错误: {ex.Message}");
            Console.WriteLine($"错误详情: {ex}");
        }

        Console.WriteLine("按任意键退出...");
        Console.ReadKey();
    }
}