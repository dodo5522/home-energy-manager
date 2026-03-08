import {createFileRoute} from '@tanstack/react-router';
import {Configuration} from '#/components/pages';

export const Route = createFileRoute('/config')({
  component: Configuration,
});
