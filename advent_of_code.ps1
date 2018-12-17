function get-advent-of-code {
    param (
        [int]$day
    )

    $data_dir = (Split-Path -parent $PSCommandPath) + "\data\";

    &cargo run --bin "day-$($day)" -- --data "$($data_dir)day_$($day).txt"
}