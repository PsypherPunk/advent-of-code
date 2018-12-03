package io.psypherpunk;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class One {

    private static final String LINE = "^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$";

    @SuppressWarnings("Duplicates")
    public static void main(String[] args) throws IOException  {
        Pattern pattern = Pattern.compile(LINE);
        int[][] sheet = new int[1000][1000];

        ClassLoader classLoader = ClassLoader.getSystemClassLoader();
        File file = new File(classLoader.getResource("input.txt").getFile());

        Matcher matcher;
        int id, left, top, width, height;
        for (String line : Files.readAllLines(file.toPath())) {

            matcher = pattern.matcher(line);
            if (!matcher.matches()) {
                System.out.println(line);
                System.exit(1);
            }
            id = Integer.parseInt(matcher.group(1));
            left = Integer.parseInt(matcher.group(2));
            top = Integer.parseInt(matcher.group(3));
            width = Integer.parseInt(matcher.group(4));
            height = Integer.parseInt(matcher.group(5));

            for (int i = left; i < left + width; i++) {
               for (int j = top; j < top + height; j++) {
                    sheet[i][j]++;
               }
            }
        }
        int squareInches = 0;
        for (int i = 0; i < 1000; i++) {
            for (int j = 0; j < 1000; j++) {
                if (sheet[i][j] > 1) {
                    squareInches++;
                }
            }
        }
        System.out.println(squareInches);
    }
}
