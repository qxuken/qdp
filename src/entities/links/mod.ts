import type { LinkSectionRes } from './LinkSection';
import { LinkSection } from './LinkSection';

let section = LinkSection.createItem({ id: 123, title: 'title', orderNumber: 1 } as LinkSectionRes);

console.log(section);
