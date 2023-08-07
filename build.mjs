import dotenv from 'dotenv';
import { build } from './build.assets.mjs';

dotenv.config();

await build();
