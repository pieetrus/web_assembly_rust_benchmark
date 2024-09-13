using ScottPlot;
using ScottPlot.Palettes;
using static BenchmarkPlotter.Utils.Constants;

namespace BenchmarkPlotter;

public class BenchmarkChartGenerator
{
    private const int width = 1000;
    private const int height = 700;
    
    private readonly (int DataSize, double WasmTime, double NativeTime)[] _releaseData = new[]
    {
        (100, 0.57728, 0.06305),
        (1000, 5.6440, 0.62375),
        (10000, 66.137, 8.9279),
        (100000, 661.36, 106.03)
    };

    private readonly (int DataSize, double WasmTime, double NativeTime)[] _debugData = new[]
    {
        (100, 2.0034, 0.08220),
        (1000, 19.668, 0.81650),
        (10000, 208.30, 10.790),
        (100000, 2079.0, 124.13)
    };
    
    // Tworzy wykres słupkowy porównujący czas wykonania WebAssembly vs Natywny Rust dla różnych rozmiarów danych w trybie Release.
    public void SaveBarChartReleaseComparison()
    {
        var plt = new Plot();
        
        plt.Title("Porównanie czasu wykonania WebAssembly vs Native Rust (Release)");
        plt.XLabel("Rozmiar danych");
        plt.YLabel("Czas wykonania (ms)");
        
        
        Category10 palette = new();
        
        Bar[] bars =
        [
            // 100
            new() { Position = 1, Value = _releaseData.First(x => x.DataSize == 100).WasmTime, FillColor = palette.GetColor(0)}, 
            new() { Position = 2, Value = _releaseData.First(x => x.DataSize == 100).NativeTime, FillColor = palette.GetColor(1)},
            
            // 1000
            new() { Position = 4, Value = _releaseData.First(x => x.DataSize == 1000).WasmTime, FillColor = palette.GetColor(0)}, 
            new() { Position = 5, Value = _releaseData.First(x => x.DataSize == 1000).NativeTime, FillColor = palette.GetColor(1)},
            
            // 10_000
            new() { Position = 7, Value = _releaseData.First(x => x.DataSize == 10_000).WasmTime, FillColor = palette.GetColor(0)}, 
            new() { Position = 8, Value = _releaseData.First(x => x.DataSize == 10_000).NativeTime, FillColor = palette.GetColor(1)},
            
            // 100_000
            new() { Position = 10, Value = _releaseData.First(x => x.DataSize == 100_000).WasmTime, FillColor = palette.GetColor(0)}, 
            new() { Position = 11, Value = _releaseData.First(x => x.DataSize == 100_000).NativeTime, FillColor = palette.GetColor(1)},
        ];
        
        // build the legend manually
        plt.Legend.IsVisible = true;
        plt.Legend.Alignment = Alignment.UpperLeft;
        plt.Legend.ManualItems.Add(new() { LabelText = "WebAssembly", FillColor = palette.GetColor(0) });
        plt.Legend.ManualItems.Add(new() { LabelText = "Natywny Rust", FillColor = palette.GetColor(1) });
        
        plt.Add.Bars(bars);
        
        // tell the plot to autoscale with no padding beneath the bars
        plt.Axes.Margins(bottom: 0);

        // show group labels on the bottom axis
        Tick[] ticks =
        [
            new(1.5, "100"),
            new(4.5, "1000"),
            new(7.5, "10_000"),
            new(10.5, "100_000")
        ];
      
        plt.Axes.Bottom.TickGenerator = new ScottPlot.TickGenerators.NumericManual(ticks);
        
        plt.SavePng($"{ChartsOutputDirectoryPath}/benchmark_release_bar_chart.png", width, height);    
    }

    // Generuje wykres liniowy pokazujący skalowanie czasu wykonania w zależności od rozmiaru danych dla obu implementacji w trybie Release.
    public void SaveLineChartScaling()
    {
        var plt = new Plot();
    
        var dataSizes = _releaseData.Select(d => (double)d.DataSize).ToArray();
        var wasmTimes = _releaseData.Select(d => d.WasmTime).ToArray();
        var nativeTimes = _releaseData.Select(d => d.NativeTime).ToArray();
    
        var sctr1 = plt.Add.Scatter(dataSizes, wasmTimes);
        sctr1.LegendText = "WebAssembly";
        var sctr2 = plt.Add.Scatter(dataSizes, nativeTimes);
        sctr2.LegendText = "Natywny Rust";
    
        plt.XLabel("Rozmiar danych");
        plt.YLabel("Czas wykonania (ms)");
        plt.Title("Skalowanie czasu wykonania (Release)");
        
        plt.ShowLegend();
        
        plt.SavePng($"{ChartsOutputDirectoryPath}/benchmark_time_per_data_size.png", width, height);    
    }

    // Tworzy wykres słupkowy porównujący wpływ optymalizacji (Debug vs Release) na obie implementacje dla rozmiaru danych 10,000 elementów.
    // public void SaveBarChartOptimizationComparison(string filePath)
    // {
    //     var plt = new Plot(600, 400);
    //
    //     double[] positions = { 0, 1, 2, 3 };
    //     var dataFor10k = new[]
    //     {
    //         _debugData.First(d => d.DataSize == 10000),
    //         _releaseData.First(d => d.DataSize == 10000)
    //     };
    //
    //     double[] wasmTimes = { dataFor10k[0].WasmTime, dataFor10k[1].WasmTime };
    //     double[] nativeTimes = { dataFor10k[0].NativeTime, dataFor10k[1].NativeTime };
    //
    //     var barWasm = plt.AddBar(wasmTimes, new[] { 0d, 1d });
    //     var barNative = plt.AddBar(nativeTimes, new[] { 0.3, 1.3 });
    //
    //     barWasm.Label = "WebAssembly";
    //     barNative.Label = "Native Rust";
    //
    //     plt.XTicks(new[] { 0.15, 1.15 }, new[] { "Debug", "Release" });
    //     plt.XLabel("Tryb kompilacji");
    //     plt.YLabel("Czas wykonania (ms)");
    //     plt.Title("Wpływ optymalizacji na wydajność (10,000 elementów)");
    //     plt.Legend();
    //
    //     plt.SaveFig(filePath);
    // }

    // Generuje wykres kołowy ilustrujący procentową różnicę w wydajności między WebAssembly a natywnym Rustem dla różnych rozmiarów danych w trybie Release.
    // public void SavePieChartPerformanceDifference(string filePath)
    // {
    //     var plt = new Plot(600, 400);
    //
    //     var performanceDifferences = _releaseData.Select(d => (d.WasmTime / d.NativeTime - 1) * 100).ToArray();
    //     var labels = _releaseData.Select(d => d.DataSize.ToString()).ToArray();
    //
    //     plt.AddPie(performanceDifferences);
    //     plt.Legend(labels);
    //
    //     plt.Title("Procentowa różnica w wydajności\nWebAssembly vs Native Rust (Release)");
    //
    //     plt.SaveFig(filePath);
    // }
}