using BenchmarkPlotter.Models;
using ScottPlot;

namespace BenchmarkPlotter.Utils;

public static class TickExtensions
{
    public static double GetPosition(this Tick[] ticks, MemoryUsageData x)
    {
        return ticks.First(t => t.Label == x.Size.ToString()).Position;
    }
    
}