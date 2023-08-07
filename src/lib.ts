import 'htmx.org';
import './styles/global.css';

if (typeof LIVE_RELOAD_URL === 'string') {
  import('./utils/liveReload.ts').then(({ run }) => run());
}
