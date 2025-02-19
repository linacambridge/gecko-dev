/* -*- js-indent-level: 2; indent-tabs-mode: nil -*- */
/* Any copyright is dedicated to the Public Domain.
   http://creativecommons.org/publicdomain/zero/1.0/ */
/* eslint-disable no-shadow, max-nested-callbacks */

"use strict";

// Test that we can detect nested event loops in tabs with the same URL.

var gClient1, gClient2, gThreadFront1, gThreadFront2;

function run_test() {
  initTestDebuggerServer();
  addTestGlobal("test-nesting1");
  addTestGlobal("test-nesting1");
  // Conect the first client to the first debuggee.
  gClient1 = new DebuggerClient(DebuggerServer.connectPipe());
  gClient1.connect(function() {
    attachTestThread(gClient1, "test-nesting1", function(
      response,
      targetFront,
      threadFront
    ) {
      gThreadFront1 = threadFront;
      start_second_connection();
    });
  });
  do_test_pending();
}

function start_second_connection() {
  gClient2 = new DebuggerClient(DebuggerServer.connectPipe());
  gClient2.connect(function() {
    attachTestThread(gClient2, "test-nesting1", function(
      response,
      targetFront,
      threadFront
    ) {
      gThreadFront2 = threadFront;
      test_nesting();
    });
  });
}

async function test_nesting() {
  let result;
  try {
    result = await gThreadFront1.resume();
  } catch (e) {
    Assert.ok(e.includes("wrongOrder"), "rejects with the wrong order");
  }
  Assert.ok(!result, "no response");

  result = await gThreadFront2.resume();
  Assert.ok(true, "resumed as expected");

  gThreadFront1.resume().then(response => {
    Assert.ok(true, "resumed as expected");

    gClient1.close(() => finishClient(gClient2));
  });
}
