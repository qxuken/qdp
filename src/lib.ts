import 'htmx.org';
import './styles/global.css';
import { connectLiveReload } from './utils/liveReload.ts';

if (APPLICATION_MODE === 'development' && typeof LIVE_RELOAD_URL === 'string') {
  connectLiveReload(LIVE_RELOAD_URL);
}
