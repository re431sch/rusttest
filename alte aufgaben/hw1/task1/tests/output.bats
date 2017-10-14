#!/usr/bin/env bats


@test "Check that we have a debug output" {
    run stat $BATS_TEST_DIRNAME/../target/debug/task1
    [ $status = 0 ]
}

@test "Output complete and correct formated" {
    run $BATS_TEST_DIRNAME/../target/debug/task1
    [[ ${lines[0]} =~ "1900" ]]
    [[ ${lines[1]} =~ "2000*" ]]
    [[ ${lines[2]} =~ "1997" ]]
    [[ ${lines[3]} =~ "1004*" ]]
    [[ ${lines[4]} =~ "1833" ]]
    [[ ${lines[5]} =~ "2016*" ]]
}

# wc output with white spaces is trimmed by xargs
@test "Output must be exact 6 lines long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task1' | wc -l | xargs"
    [ "$output" = "6" ]
}
