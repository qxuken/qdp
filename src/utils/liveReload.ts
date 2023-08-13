export function connectLiveReload(sourceUrl: string) {
  let source = new EventSource(sourceUrl, { withCredentials: true });
  source.addEventListener('update', () => location.reload());
}
