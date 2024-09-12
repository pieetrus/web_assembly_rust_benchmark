namespace BenchmarkPlotter.Utils;

public static class UnitConverter
{
    // https://www.geeksforgeeks.org/understanding-file-sizes-bytes-kb-mb-gb-tb-pb-eb-zb-yb/
    public static double BytesToMegabytes(double bytes) => bytes / (1024 * 1024);
    public static double MegabytesToBytes(double megabytes) => megabytes * (1024 * 1024);
}