using System;
using System.IO;
using System.Linq;

namespace two
{
    class Program
    {
        static void Main(string[] args)
        {
            var coordinates = File
                .ReadAllLines(Path.Combine(Directory.GetCurrentDirectory(), "input.txt"))
                .Select(c => c.Trim().Split(new[] { ", " }, StringSplitOptions.None))
                .Select(c => c.Select(i => Convert.ToInt32(i)).ToArray())
                .Select(c => (x: c[0], y: c[1]))
                .ToArray();

            var width = coordinates.Max(c => c.x);
            var height = coordinates.Max(c => c.y);

            var safeCoords = 0;

            for (int x = 0; x < width; x++)
            {
                for (int y = 0; y < height; y++)
                {
                    var distances = coordinates
                        .Select((c, i) => (index: i, dist: Math.Abs(c.x - x) + Math.Abs(c.y - y)))
                        .OrderBy(c => c.dist)
                        .ToArray();

                    if (distances.Sum(c => c.dist) < 10000)
                    {
                        safeCoords++;
                    }
                }
            }
            Console.WriteLine(safeCoords);
        }
    }
}
