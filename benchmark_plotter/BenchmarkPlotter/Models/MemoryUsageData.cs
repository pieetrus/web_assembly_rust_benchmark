namespace BenchmarkPlotter.Models;

public class MemoryUsageData
{
    public string Name { get; set; } = null!;
    public List<Snapshot> Snapshots { get; set; } = [];

    public double  MaxMemoryUsageMB => Snapshots.Max(s => s.MemoryUsageMB);
    public double  TimeToMaxMemory => Snapshots.FindIndex(s => s.MemoryUsageMB == MaxMemoryUsageMB); // todo: ensure that this is correct
    public double AverageMemoryUsageMB => Snapshots.Average(s => s.MemoryUsageMB);
}