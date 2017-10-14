#!/usr/bin/env bats


@test "Check that we have a debug output" {
    run stat $BATS_TEST_DIRNAME/../target/debug/task3
    [ $status = 0 ]
}

@test "Output complete and correct formated" {
    run $BATS_TEST_DIRNAME/../target/debug/task3
    [[ ${lines[0]} =~ "In '♥ The quick brown fox jumps over the lazy dog. ♥', 3 'e' were found" ]]
}

# wc output with white spaces is trimmed by xargs
@test "Output must be exact 1 line long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task3' | wc -l | xargs"
    [ "$output" = "1" ]
}
