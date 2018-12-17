function advent-of-code {
    param (
        [int]$day
    )

    $data = "";

    $data_dir = Split-Path -parent $PSCommandPath;

    switch ($day) {
        1 { $data = "$($data_dir)/data/day_1.txt" }
        2 { $data = "$($dataa_dir)/data/day_2.txt"}
    }

    if ($data -eq "") {
        &cargo run --bin "day-$($day)"
    } else {
        &cargo run --bin "day-$($day)" -- --data="$($data)"
    }
}