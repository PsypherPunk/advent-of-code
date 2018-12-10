#!/usr/bin/env perl

use strict;
use warnings;

open my $fh, "<", "input.txt" or die;
my $input = do { local $/; <$fh> };

my @nodes = split(/\s+/, $input);

sub read_licence {
    my $node_value = 0;
    my @child_values = ();
    my $child_count = shift @nodes;
    my $metadata_count = shift @nodes;

    for (1..$child_count) {
        push @child_values, read_licence();
    }
    if ($child_count == 0) {
        for(1..$metadata_count) {
            $node_value += shift @nodes;
        }
    } else {
        for(1..$metadata_count) {
            my $metadata_index = shift @nodes;
            if ($metadata_index <= scalar(@child_values)) {
                $node_value += $child_values[$metadata_index - 1];
            }
        }
    }
    return $node_value;
}

print read_licence();

