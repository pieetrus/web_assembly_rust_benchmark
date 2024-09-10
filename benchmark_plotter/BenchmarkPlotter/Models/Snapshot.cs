namespace BenchmarkPlotter.Models;

public class Snapshot
{
    public long Time { get; set; }
    public double MemoryUsageB { get; set; }
    public double MemoryUsageMB  => MemoryUsageB / (1024 * 1024); //todo: decide how to calculate this
}