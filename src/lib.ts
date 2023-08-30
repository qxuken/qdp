import * as htmx from 'htmx.org';
import './styles/global.css';

if (APPLICATION_MODE === 'development') {
  htmx.logAll();
}
