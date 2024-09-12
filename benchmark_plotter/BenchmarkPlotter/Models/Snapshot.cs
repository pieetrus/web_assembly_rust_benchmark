using BenchmarkPlotter.Utils;

namespace BenchmarkPlotter.Models;

public class Snapshot
{
    public long InstructionsExecuted { get; set; }
    public double MemoryUsageB { get; set; }
    public double MemoryUsageMB  => UnitConverter.BytesToMegabytes(MemoryUsageB);
}