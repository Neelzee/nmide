# IDE-Explorer

Adds an event listener on `open-project`, and expects the event argument to
be of this structure:

```typescript
type File = {
  obj: {
    file: { obj: { path: { str: string } } }
  }
}

type Folder = {
  obj: {
    folder: { obj: { path: { str: string } }, contents: (File | Folder)[] }
  }
}

type Argument = {
  obj: {
    folder?: Folder,
    file?: File,
  }
};
```

If the input is a folder called `root`, with two files, and a sub-folder with
one file, it will be transformed into this html structure:

```html
<div class="file-explorer">
  <div id="root" class="folder">
    root
    <span id="root/foobar.txt" class="file">
      foobar.txt
    </span>
    <span id="root/foo.txt" class="file">
      foo.txt
    </span>
    <div id="root/bar" class="folder">
      <span id="root/bar/foo.txt" class="file">
        foo.txt
      </span>
    </div>
  </div>
</div>
```