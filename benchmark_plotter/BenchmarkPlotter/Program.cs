using BenchmarkPlotter;
using static BenchmarkPlotter.Constants;

var allData = DataLoader.LoadData(InputFilePath);

Console.WriteLine("Data loaded successfully.");

foreach (var data in allData)
{
    Console.WriteLine($"Filename: {data.Name}");
    Console.WriteLine($"Max Memory Usage: {data.MaxMemoryUsageMB:F2} MB");
    Console.WriteLine($"Time to Max Memory: {data.TimeToMaxMemory}");
    Console.WriteLine($"Average Memory Usage: {data.AverageMemoryUsageMB:F2} MB");
    Console.WriteLine();
}