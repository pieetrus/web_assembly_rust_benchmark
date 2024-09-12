using BenchmarkPlotter;
using BenchmarkPlotter.Models;
using BenchmarkPlotter.Utils;

using static BenchmarkPlotter.Utils.Constants;

var allData = DataLoader.LoadData(InputFilePath);

// Średnie zużycie pamięci w trakcie foreach (var data in allData) działania programu  dla każdego rozmiaru danych
var averageMemoryUsageMB = allData.First(x => x is {Size: 1_00, Platform: Platform.NAIVE, Mode: Mode.RELEASE})
    .Snapshots.Average(snapshot => snapshot.MemoryUsageMB);

Console.WriteLine($"Average memory usage: {averageMemoryUsageMB} MB");

