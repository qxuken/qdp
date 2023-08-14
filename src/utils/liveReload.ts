export function connectLiveReload(sourceUrl: string) {
  let connected = false;
  let source = new EventSource(sourceUrl, { withCredentials: true });
  source.addEventListener('init', () => {
    if (connected) {
      location.reload();
    }
    connected = true;
  });
  source.addEventListener('update', () => location.reload());
}
