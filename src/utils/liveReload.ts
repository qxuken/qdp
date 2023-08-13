export function run() {
  if (typeof LIVE_RELOAD_URL !== 'string') {
    return;
  }
  let source = new EventSource(LIVE_RELOAD_URL, { withCredentials: true });
  source.addEventListener('update', () => location.reload());
}
