using BenchmarkPlotter.Utils;

namespace BenchmarkPlotter.UnitTests;

public class UnitConverterTests
{
    [Fact]
    public void Should_BytesToMegabytes_ReturnProperValue()
    {
        var byteValue = 1024 * 1024;
        var expectedMegabyteValue = 1;
        
        var result = UnitConverter.BytesToMegabytes(byteValue);
        
        Assert.Equal(expectedMegabyteValue, result);
    }
    
    [Fact]
    public void Should_BytesToMegabytes_ReturnProperValue2()
    {
        double byteValue = 104857600;
        var expectedMegabyteValue = 100;
        
        var result = UnitConverter.BytesToMegabytes(byteValue);
        
        Assert.Equal(expectedMegabyteValue, result);
    }
    
    [Fact]
    public void Should_BytesToMegabytes_ReturnProperValue3()
    {
        double byteValue = 1048576000;
        var expectedMegabyteValue = 1000;
        
        var result = UnitConverter.BytesToMegabytes(byteValue);
        
        Assert.Equal(expectedMegabyteValue, result);
    }
    
    [Fact]
    public void Should_BytesToMegabytes_ReturnProperValue4()
    {
        double byteValue = 10485760000;
        var expectedMegabyteValue = 10000;
        
        var result = UnitConverter.BytesToMegabytes(byteValue);
        
        Assert.Equal(expectedMegabyteValue, result);
    }
    
}