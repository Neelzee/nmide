interface Action {
  public Action addText(String s);
  public Action removeText(String s);
  public Action redoAction(Action action);
  public Action undoAction(Action action);
  public Action combineAction(Action first, Action second);
}
