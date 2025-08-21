concept Log = {
  type Log;
  function appendLog(first: Log, second: Log): Log;
  axiom logComposition(a: Log, B: Log, c: Log) {
    assert
      appendLog(appendLog(a, b), c)
        ==
      appendLog(a, appendLog(b, c));
  }
};
