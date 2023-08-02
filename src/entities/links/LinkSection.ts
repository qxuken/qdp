import { z } from 'zod';
import { createResource } from '../resource';

const resourceUrl = `/api/links/section`;

const linkSectionResSchema = z.object({
  id: z.number(),
  title: z.string(),
  orderNumber: z.number(),
});

export type LinkSectionRes = z.infer<typeof linkSectionResSchema>;
type NewLinkSection = Omit<LinkSectionRes, 'id' | 'items'>;

export const LinkSection = createResource<LinkSectionRes, NewLinkSection>(
  resourceUrl,
  linkSectionResSchema.parse,
);
