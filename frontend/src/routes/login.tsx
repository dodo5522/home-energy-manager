import {createFileRoute, Navigate} from '@tanstack/react-router';
import type {SubmitEvent} from 'react';
import {useState} from 'react';
import {authClient} from '#/lib/auth-client';

type LoginSearch = {
  redirect?: string;
};

const resolveRedirect = (redirect?: string) => {
  if (typeof redirect !== 'string') {
    return '/';
  }

  if (!redirect.startsWith('/')) {
    return '/';
  }

  return redirect;
};

const LoginPage = () => {
  const {redirect} = Route.useSearch();
  const redirectTo = resolveRedirect(redirect);

  const {data: session, isPending} = authClient.useSession();
  const [isSignUp, setIsSignUp] = useState(false);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  if (isPending) {
    return (
      <div className="flex items-center justify-center py-10">
        <div
          className="h-5 w-5 animate-spin rounded-full border-2 border-neutral-200 border-t-neutral-900 dark:border-neutral-800 dark:border-t-neutral-100"/>
      </div>
    );
  }

  if (session?.user) {
    return <Navigate to={redirectTo} replace/>;
  }

  const handleSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      if (isSignUp) {
        const result = await authClient.signUp.email({
          email,
          password,
          name,
        });
        if (result.error) {
          setError(result.error.message || 'Sign up failed');
          return;
        }
      } else {
        const result = await authClient.signIn.email({
          email,
          password,
        });
        if (result.error) {
          setError(result.error.message || 'Sign in failed');
          return;
        }
      }
    } catch (_) {
      setError('An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex justify-center py-10 px-4">
      <div className="w-full max-w-md p-6">
        <h1 className="text-lg font-semibold leading-none tracking-tight">
          {isSignUp ? 'Create an account' : 'Sign in'}
        </h1>
        <p className="text-sm text-neutral-500 dark:text-neutral-400 mt-2 mb-6">
          {isSignUp
            ? 'Enter your information to create an account'
            : 'Sign in to open the protected page'}
        </p>

        <form onSubmit={handleSubmit} className="grid gap-4">
          {isSignUp && (
            <div className="grid gap-2">
              <label
                htmlFor="name"
                className="text-sm font-medium leading-none"
              >
                Name
              </label>
              {/** biome-ignore lint/correctness/useUniqueElementIds: template form */}
              <input
                id="name"
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="flex h-9 w-full border border-neutral-300 dark:border-neutral-700 bg-transparent px-3 text-sm focus:outline-none focus:border-neutral-900 dark:focus:border-neutral-100 disabled:cursor-not-allowed disabled:opacity-50"
                required
              />
            </div>
          )}

          <div className="grid gap-2">
            <label htmlFor="email" className="text-sm font-medium leading-none">
              Email
            </label>
            {/** biome-ignore lint/correctness/useUniqueElementIds: template form */}
            <input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="flex h-9 w-full border border-neutral-300 dark:border-neutral-700 bg-transparent px-3 text-sm focus:outline-none focus:border-neutral-900 dark:focus:border-neutral-100 disabled:cursor-not-allowed disabled:opacity-50"
              required
            />
          </div>

          <div className="grid gap-2">
            <label
              htmlFor="password"
              className="text-sm font-medium leading-none"
            >
              Password
            </label>
            {/** biome-ignore lint/correctness/useUniqueElementIds: template form */}
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="flex h-9 w-full border border-neutral-300 dark:border-neutral-700 bg-transparent px-3 text-sm focus:outline-none focus:border-neutral-900 dark:focus:border-neutral-100 disabled:cursor-not-allowed disabled:opacity-50"
              required
              minLength={8}
            />
          </div>

          {error && (
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-3">
              <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
            </div>
          )}

          <button
            type="submit"
            disabled={loading}
            className="w-full h-9 px-4 text-sm font-medium text-white bg-neutral-900 hover:bg-neutral-800 dark:bg-neutral-50 dark:text-neutral-900 dark:hover:bg-neutral-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? (
              <span className="flex items-center justify-center gap-2">
                <span
                  className="h-4 w-4 animate-spin rounded-full border-2 border-neutral-400 border-t-white dark:border-neutral-600 dark:border-t-neutral-900"/>
                <span>Please wait</span>
              </span>
            ) : isSignUp ? (
              'Create account'
            ) : (
              'Sign in'
            )}
          </button>
        </form>

        <div className="mt-4 text-center">
          <button
            type="button"
            onClick={() => {
              setIsSignUp(!isSignUp);
              setError('');
            }}
            className="text-sm text-neutral-500 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-neutral-100 transition-colors"
          >
            {isSignUp
              ? 'Already have an account? Sign in'
              : "Don't have an account? Sign up"}
          </button>
        </div>
      </div>
    </div>
  );
};

export const Route = createFileRoute('/login')({
  validateSearch: (search: Record<string, unknown>): LoginSearch => ({
    redirect: typeof search.redirect === 'string' ? search.redirect : undefined,
  }),
  component: LoginPage,
});
