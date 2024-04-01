export type FolderOrFile = Folder | File;

export type File = {
  name: string,
  extension: string,
  path: string
};

export type Folder = {
  name: string,
  path: string,
  content: FolderOrFile[]
};