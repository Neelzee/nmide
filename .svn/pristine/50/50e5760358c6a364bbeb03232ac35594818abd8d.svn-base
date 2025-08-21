trait Action;

trait Editor {
  pub fn addText() -> Self;
  pub fn removeText() -> Self;
  pub fn redoAction(&self) -> Self;
  pub fn undoAction(&self) -> Self;
  pub fn combineAction(self, second: Self) -> Self;
}
