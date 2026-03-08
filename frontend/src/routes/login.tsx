import {createFileRoute} from '@tanstack/react-router';
import {LogIn} from '#/components/pages';
import type {LoginSearch} from '#/types';

const resolveRedirect = ({redirect}: LoginSearch): string => {
  if (!redirect?.startsWith('/')) {
    return '/';
  }
  return redirect;
};

const LogInRoot = () => {
  const search = Route.useSearch();
  return <LogIn redirectTo={resolveRedirect(search)}/>;
};

export const Route = createFileRoute('/login')({
  validateSearch: (search: Record<string, unknown>): LoginSearch => ({
    redirect: typeof search.redirect === 'string' ? search.redirect : undefined,
  }),
  component: LogInRoot,
});
