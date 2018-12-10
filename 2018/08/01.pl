#!/usr/bin/env perl

use strict;
use warnings;

open my $fh, "<", "input.txt" or die;
my $input = do { local $/; <$fh> };

my @nodes = split(/\s+/, $input);
my $metadata_sum = 0;

sub read_licence {
    my $child_count = shift @nodes;
    my $metadata_count = shift @nodes;

    for (1..$child_count) {
        read_licence();
    }
    for(1..$metadata_count) {
        $metadata_sum += shift @nodes;
    }
}

read_licence();
print $metadata_sum;

