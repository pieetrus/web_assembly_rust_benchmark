using BenchmarkPlotter.Models;
using ScottPlot;
using ScottPlot.Palettes;
using static BenchmarkPlotter.Utils.Constants;
using Platform = BenchmarkPlotter.Models.Platform;

// multiplot: https://scottplot.net/faq/multiplot/
namespace BenchmarkPlotter;

public static class ChartGenerator
{
    private const int width = 1000;
    private const int height = 700;

    public static void CreateInstructionsToMaxMemoryUsageChart(List<MemoryUsageData> allData)
    {
        var plt = new Plot();
        plt.Title("Instrukcje do maksymalnego zużycia pamięci dla różnych rozmiarów danych");
        plt.XLabel("Rozmiar danych wejściowych (ilość elementów)");
        plt.YLabel("Instrukcje do maksymalnego zużycia pamięci");
        
        Category10 palette = new();
        
        Bar[] bars =
        [
            // 100
            new() { Position = 1, Value = allData.First(x => x is { Size: 100, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 2, Value = allData.First(x => x is { Size: 100, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 3, Value = allData.First(x => x is { Size: 100, Platform: Platform.WASM, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 4, Value = allData.First(x => x is { Size: 100, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)}, //wasm - release
            
            // 1000
            new() { Position = 6, Value = allData.First(x => x is { Size: 1000, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 7, Value = allData.First(x => x is { Size: 1000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 8, Value = allData.First(x => x is { Size: 1000, Platform: Platform.WASM, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 9, Value = allData.First(x => x is { Size: 1000, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)}, //wasm - release

            // 10_000
            new() { Position = 11, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 12, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 13, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.WASM, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 14, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)}, //wasm - release

            // 100_000
            new() { Position = 16, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 17, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 18, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.WASM, Mode: Mode.DEBUG }).InstructionsToMaxMemory, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 19, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)} //wasm - release
        ];
        
        plt.Add.Bars(bars);
        
        // build the legend manually
        plt.Legend.IsVisible = true;
        plt.Legend.Alignment = Alignment.UpperLeft;
        plt.Legend.ManualItems.Add(new() { LabelText = "Natywny - Debug", FillColor = palette.GetColor(0) });
        plt.Legend.ManualItems.Add(new() { LabelText = "Natywny - Release", FillColor = palette.GetColor(1) });
        plt.Legend.ManualItems.Add(new() { LabelText = "WebAssembly - Debug", FillColor = palette.GetColor(2) });
        plt.Legend.ManualItems.Add(new() { LabelText = "WebAssembly - Release", FillColor = palette.GetColor(3) });
        
        // show group labels on the bottom axis
        Tick[] ticks =
        [
            new(3, "100"),
            new(8, "1000"),
            new(13, "10_000"),
            new(18, "100_000")
        ];
        
        
        plt.Axes.Bottom.TickGenerator = new ScottPlot.TickGenerators.NumericManual(ticks);
        plt.Axes.Bottom.MajorTickStyle.Length = 0;
        plt.HideGrid();

        // tell the plot to autoscale with no padding beneath the bars
        plt.Axes.Margins(bottom: 0);

        plt.SavePng($"{ChartsOutputDirectoryPath}/instructions_to_max_memory_usage_all_chart.png", width, height);
    }

    public static void CreateInstructionsToMaxMemoryUsageChartReleaseOnly(List<MemoryUsageData> allData)
    {
        var plt = new Plot();
        plt.Title("Instrukcje do maksymalnego zużycia pamięci dla różnych rozmiarów danych");
        plt.XLabel("Rozmiar danych wejściowych (ilość elementów)");
        plt.YLabel("Instrukcje do maksymalnego zużycia pamięci");
        
        Category10 palette = new();
        
        Bar[] bars =
        [
            // 100
            new() { Position = 1, Value = allData.First(x => x is { Size: 100, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 2, Value = allData.First(x => x is { Size: 100, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)}, //wasm - release
            
            // 1000
            new() { Position = 4, Value = allData.First(x => x is { Size: 1000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 5, Value = allData.First(x => x is { Size: 1000, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)}, //wasm - release

            // 10_000
            new() { Position = 7, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 8, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)}, //wasm - release

            // 100_000
            new() { Position = 10, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 11, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.WASM, Mode: Mode.RELEASE }).InstructionsToMaxMemory, FillColor = palette.GetColor(3)} //wasm - release
        ];
        
        plt.Add.Bars(bars);
        
        // build the legend manually
        plt.Legend.IsVisible = true;
        plt.Legend.Alignment = Alignment.UpperLeft;
        plt.Legend.ManualItems.Add(new() { LabelText = "Natywny - Release", FillColor = palette.GetColor(1) });
        plt.Legend.ManualItems.Add(new() { LabelText = "WebAssembly - Release", FillColor = palette.GetColor(3) });
        
        // show group labels on the bottom axis
        Tick[] ticks =
        [
            new(1, "100"),
            new(4, "1000"),
            new(7, "10_000"),
            new(10, "100_000")
        ];
        
        
        plt.Axes.Bottom.TickGenerator = new ScottPlot.TickGenerators.NumericManual(ticks);
        plt.Axes.Bottom.MajorTickStyle.Length = 0;
        plt.HideGrid();

        // tell the plot to autoscale with no padding beneath the bars
        plt.Axes.Margins(bottom: 0);

        plt.SavePng($"{ChartsOutputDirectoryPath}/instructions_to_max_memory_usage_release_chart.png", width, height);
    }

    // https://scottplot.net/cookbook/5.0/Bar/GroupedBarPlot
    public static void CreateMaxMemoryUsageChart(List<MemoryUsageData> allData)
    {
        var plt = new Plot();
        plt.Title("Maksymalne zużycie pamięci (MB) dla różnych rozmiarów danych");
        plt.XLabel("Rozmiar danych wejściowych (ilość elementów)");
        plt.YLabel("Maksymalne zużycie pamięci (MB)");
        
        Category10 palette = new();
        
        Bar[] bars =
        [
            // 100
            new() { Position = 1, Value = allData.First(x => x is { Size: 100, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 2, Value = allData.First(x => x is { Size: 100, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 3, Value = allData.First(x => x is { Size: 100, Platform: Platform.WASM, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 4, Value = allData.First(x => x is { Size: 100, Platform: Platform.WASM, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(3)}, //wasm - release
            
            // 1000
            new() { Position = 6, Value = allData.First(x => x is { Size: 1000, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 7, Value = allData.First(x => x is { Size: 1000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 8, Value = allData.First(x => x is { Size: 1000, Platform: Platform.WASM, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 9, Value = allData.First(x => x is { Size: 1000, Platform: Platform.WASM, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(3)}, //wasm - release

            // 10_000
            new() { Position = 11, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 12, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 13, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.WASM, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 14, Value = allData.First(x => x is { Size: 10_000, Platform: Platform.WASM, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(3)}, //wasm - release

            // 100_000
            new() { Position = 16, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.NAIVE, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(0)}, //naive - debug
            new() { Position = 17, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.NAIVE, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(1)}, //naive - release
            new() { Position = 18, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.WASM, Mode: Mode.DEBUG }).MaxMemoryUsageMB, FillColor = palette.GetColor(2)}, //wasm - debug
            new() { Position = 19, Value = allData.First(x => x is { Size: 100_000, Platform: Platform.WASM, Mode: Mode.RELEASE }).MaxMemoryUsageMB, FillColor = palette.GetColor(3)} //wasm - release
        ];
        
        plt.Add.Bars(bars);
        
        // build the legend manually
        plt.Legend.IsVisible = true;
        plt.Legend.Alignment = Alignment.UpperLeft;
        plt.Legend.ManualItems.Add(new() { LabelText = "Natywny - Debug", FillColor = palette.GetColor(0) });
        plt.Legend.ManualItems.Add(new() { LabelText = "Natywny - Release", FillColor = palette.GetColor(1) });
        plt.Legend.ManualItems.Add(new() { LabelText = "WebAssembly - Debug", FillColor = palette.GetColor(2) });
        plt.Legend.ManualItems.Add(new() { LabelText = "WebAssembly - Release", FillColor = palette.GetColor(3) });
        
        // show group labels on the bottom axis
        Tick[] ticks =
        [
            new(3, "100"),
            new(8, "1000"),
            new(13, "10_000"),
            new(18, "100_000")
        ];
        
        plt.Axes.Bottom.TickGenerator = new ScottPlot.TickGenerators.NumericManual(ticks);
        plt.Axes.Bottom.MajorTickStyle.Length = 0;
        plt.HideGrid();

        // tell the plot to autoscale with no padding beneath the bars
        plt.Axes.Margins(bottom: 0);

        plt.SavePng($"{ChartsOutputDirectoryPath}/max_memory_usage_all_chart.png", width, height);
    }
}