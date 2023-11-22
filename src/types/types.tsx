export type File = {
  name: String;
  path: String;
};

export type Folder = {
  name: String;
  path: String;
  contents: [];
};

export type FolderOrFile = Folder | File;
