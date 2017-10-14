#!/usr/bin/env bats


@test "Check that we have a debug output" {
    run stat $BATS_TEST_DIRNAME/../target/debug/task2
    [ $status = 0 ]
}

@test "Output complete and correct formated" {
    run $BATS_TEST_DIRNAME/../target/debug/task2
    [[ ${lines[0]} =~ "Grains Total: 18446744073709551615" ]]
}

# wc output with white spaces is trimmed by xargs
@test "Output must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task2' | wc -l | xargs"
    [ "$output" = "1" ]
}
