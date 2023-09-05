import * as htmx from 'htmx.org';
import './utils/disableInput';

import './styles/global.css';

if (APPLICATION_MODE === 'development') {
  htmx.logAll();
}
