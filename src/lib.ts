import 'htmx.org';
import './styles/global.css';

if (APPLICATION_MODE === 'development' && typeof LIVE_RELOAD_URL === 'string') {
  import('./utils/liveReload.ts').then(({ run }) => run());
}
