using BenchmarkPlotter.Models;

namespace BenchmarkPlotter.Utils;

public static class DataLoader
{
    public static List<MemoryUsageData> LoadData(string filePath)
    {
        var result = new List<MemoryUsageData>();
        var currentData = new MemoryUsageData();;

        foreach (var line in File.ReadLines(filePath))
        {
            if (line.StartsWith("filename:"))
            {
                var currentFileName = line.Split(':')[1].Trim();
                var benchmarkName = currentFileName.Split("out.")[1];
                
                currentData = new MemoryUsageData { Name = benchmarkName };
            }
            else if (line.StartsWith("time="))
            {
                var timeValue = long.Parse(line.Split('=')[1]);
                currentData.Snapshots.Add(new Snapshot { InstructionsExecuted = timeValue });            
            }
            else if (line.StartsWith("mem_heap_B="))
            {
                var mem_heap_B_value = long.Parse(line.Split('=')[1]);
                if (currentData.Snapshots.Count > 0)
                {
                    currentData.Snapshots[^1].MemoryUsageB = mem_heap_B_value;
                }
            }
            else if (line.StartsWith("----"))
            {
                result.Add(currentData);
            }
        }

        return result;
    }
}