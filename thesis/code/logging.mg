trait Log {
    pub fn appendLog(self, other: Self) -> Self;
}

concept Log = {
  type Log;
    function appendLog(first: Log, second: Log): Log;

};
