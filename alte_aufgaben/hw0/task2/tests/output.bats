#!/usr/bin/env bats


@test "Check that we have a debug output" {
    run stat $BATS_TEST_DIRNAME/../target/debug/task2
    [ $status = 0 ]
}

@test "Test that we get the collaz result" {
    run $BATS_TEST_DIRNAME/../target/debug/task2
    [[ ${lines[0]} =~ "1 -> 82" ]]
    [[ ${lines[1]} =~ "2 -> 41" ]]
    [[ ${lines[2]} =~ "3 -> 124" ]]
    [[ ${lines[3]} =~ "4 -> 62" ]]
    [[ ${lines[4]} =~ "5 -> 31" ]]
    [[ ${lines[108]} =~ "109 -> 4" ]]
    [[ ${lines[109]} =~ "110 -> 2" ]]
    [[ ${lines[110]} =~ "111 -> 1" ]]
}
