import * as htmx from 'htmx.org';
import { SyncInputSize } from './hypermediaExtensions/behaviors';
import './hypermediaExtensions/htmx';
import './styles/global.css';

if (APPLICATION_MODE === 'development') {
  htmx.logAll();
}

SyncInputSize.initOn(['htmx:load']);
