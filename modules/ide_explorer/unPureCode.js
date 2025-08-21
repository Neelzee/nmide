export function run() {
  const folders = document.querySelectorAll('.folder');
  folders.forEach(folder => {
    folder.addEventListener('click', () => {
      document.querySelectorAll('.selected').forEach(el => {
        el.classList.remove('selected');
      });

      this.classList.add('selected');
    });
  });

  const files = document.querySelectorAll('.file');
  files.forEach(file => {
    file.addEventListener('click', () => {
      document.querySelectorAll('.selected').forEach(el => {
        el.classList.remove('selected');
      });

      this.classList.add('selected');
    });
  });
}
