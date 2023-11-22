export type File = {
  name: string;
  path: string;
};

export type Folder = {
  name: string;
  path: string;
  contents: FolderOrFile[];
};

export type FolderOrFile = Folder | File;
