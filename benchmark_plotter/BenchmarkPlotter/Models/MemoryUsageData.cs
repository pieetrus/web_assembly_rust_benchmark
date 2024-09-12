namespace BenchmarkPlotter.Models;

public class MemoryUsageData
{
    public string Name { get; init; } = null!; //debug-naive-100
    public int Size => int.Parse(Name.Split('-').Last());
    public Platform Platform => GetPlatformVariant(Name.Split('-')[1]);
    public Mode Mode => GetMode(Name.Split('-').First());

    public List<Snapshot> Snapshots { get; } = [];

    public double MaxMemoryUsageMB => Snapshots.Max(s => s.MemoryUsageMB);
    private double MaxMemoryUsageB => Snapshots.Max(s => s.MemoryUsageB);
    public double InstructionsToMaxMemory => Snapshots.First(s => s.MemoryUsageB == MaxMemoryUsageB).InstructionsExecuted; // todo: ensure that this is correct
    public double AverageMemoryUsageMB => Snapshots.Average(s => s.MemoryUsageMB);
    
    private static Platform GetPlatformVariant(string variant)
    {
        return variant switch
        {
            "naive" => Platform.NAIVE,
            "wasm" => Platform.WASM,
            _ => throw new ArgumentException("Invalid variant")
        };
    }
    
    private static Mode GetMode(string mode)
    {
        return mode switch
        {
            "debug" => Mode.DEBUG,
            "release" => Mode.RELEASE,
            _ => throw new ArgumentException("Invalid mode")
        };
    }
}