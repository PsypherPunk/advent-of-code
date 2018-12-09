using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace one
{
    class One
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

            var grid = new int[width + 1, height + 1];

            for (int x = 0; x < width; x++)
            {
                for (int y = 0; y < height; y++)
                {
                    var distances = coordinates
                        .Select((c, i) => (index: i, dist: Math.Abs(c.x - x) + Math.Abs(c.y - y)))
                        .OrderBy(c => c.dist)
                        .ToArray();

                    if (distances[0].dist == distances[1].dist)
                    {
                        grid[x, y] = -1;
                    }
                    else
                    {
                        grid[x, y] = distances[0].index;
                    }
                }
            }

            var unbounded = new List<int>();
            var counter = Enumerable.Range(-1, coordinates.Length + 1).ToDictionary(i => i, _ => 0);

            for (int x = 0; x < width; x++)
            {
                for (int y = 0; y < height; y++)
                {
                    if (x == 0 || y == 0 || x > width || y > height)
                    {
                        unbounded.Add(grid[x, y]);
                    }

                    counter[grid[x, y]]++;
                }
            }

            Console.WriteLine(counter
                .Where(coordCount => !unbounded.Contains(coordCount.Key))
                .OrderByDescending(coordCount => coordCount.Value).ToArray()[0]);

        }
    }
}
