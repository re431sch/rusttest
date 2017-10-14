#!/usr/bin/env bats


@test "Check that we have a debug output" {
    run stat $BATS_TEST_DIRNAME/../target/debug/task1
    [ $status = 0 ]
}

@test "Test that we get the 'Hello Ferris Greetings'" {
    run $BATS_TEST_DIRNAME/../target/debug/task1
    [[ ${lines[0]} =~ "Hello, Ferris!" ]]
    [[ ${lines[1]} =~ "Hello, Ferris!" ]]
}
