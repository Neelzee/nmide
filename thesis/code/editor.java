interface Action {}

interface Editor {
  public Action addText();
  public Action removeText();
  public Action redoAction(Action action);
  public Action undoAction(Action action);
  public Action combineAction(Action first, Action second);
}
