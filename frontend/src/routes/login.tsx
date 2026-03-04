import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  CircularProgress,
  Stack,
  TextField,
  Typography,
} from '@mui/material';
import {createFileRoute, Navigate} from '@tanstack/react-router';
import {type SubmitEvent, useState} from 'react';
import {authClient} from '#/lib/auth-client';

type LoginSearch = {
  redirect?: string;
};

const resolveRedirect = (redirect?: string) => {
  if (!redirect?.startsWith('/')) {
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
      <Box sx={{display: 'flex', justifyContent: 'center', py: 10}}>
        <CircularProgress size={24}/>
      </Box>
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
    <Box sx={{display: 'flex', justifyContent: 'center', py: 10, px: 2}}>
      <Card sx={{width: '100%', maxWidth: 420}}>
        <CardContent>
          <Stack spacing={2.5}>
            <Box>
              <Typography variant="h6" fontWeight={600}>
                {isSignUp ? 'Create an account' : 'Sign in'}
              </Typography>
              <Typography variant="body2" color="text.secondary" sx={{mt: 1}}>
                {isSignUp
                  ? 'Enter your information to create an account'
                  : 'Sign in to open the protected page'}
              </Typography>
            </Box>

            <Box component="form" onSubmit={handleSubmit}>
              <Stack spacing={2}>
                {isSignUp && (
                  // biome-ignore lint/correctness/useUniqueElementIds: to enable auto fill in browser
                  <TextField
                    id="name"
                    label="Name"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                    fullWidth
                    size="small"
                  />
                )}
                {/*biome-ignore lint/correctness/useUniqueElementIds: to enable auto fill in browser*/}
                <TextField
                  id="email"
                  label="Email"
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  fullWidth
                  size="small"
                />
                <TextField
                  label="Password"
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  fullWidth
                  size="small"
                  inputProps={{minLength: 8}}
                />

                {error && <Alert severity="error">{error}</Alert>}

                <Button
                  type="submit"
                  disabled={loading}
                  variant="contained"
                  fullWidth
                >
                  {loading ? (
                    <Stack direction="row" spacing={1} alignItems="center">
                      <CircularProgress size={16} color="inherit"/>
                      <span>Please wait</span>
                    </Stack>
                  ) : isSignUp ? (
                    'Create account'
                  ) : (
                    'Sign in'
                  )}
                </Button>
              </Stack>
            </Box>

            <Button
              type="button"
              variant="text"
              onClick={() => {
                setIsSignUp(!isSignUp);
                setError('');
              }}
            >
              {isSignUp
                ? 'Already have an account? Sign in'
                : "Don't have an account? Sign up"}
            </Button>
          </Stack>
        </CardContent>
      </Card>
    </Box>
  );
};

export const Route = createFileRoute('/login')({
  validateSearch: (search: Record<string, unknown>): LoginSearch => ({
    redirect: typeof search.redirect === 'string' ? search.redirect : undefined,
  }),
  component: LoginPage,
});
