import { createOrderExtension, createResource } from '@entities/Resource';
import { z } from 'zod';

const resourceUrl = `/api/links/section`;

const linkSectionResSchema = z.object({
  id: z.number(),
  title: z.string(),
  orderNumber: z.number(),
});

type LinkSectionRes = z.infer<typeof linkSectionResSchema>;
type NewLinkSection = Omit<LinkSectionRes, 'id' | 'order_number' | 'items'>;

export const LinkSection = createResource<LinkSectionRes, NewLinkSection>(
  resourceUrl,
  linkSectionResSchema.parse,
  createOrderExtension,
);
