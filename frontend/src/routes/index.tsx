import {
  Avatar,
  Box,
  Button,
  Card,
  CardContent,
  CircularProgress,
  Stack,
  Typography,
} from '@mui/material';
import {createFileRoute, Navigate} from '@tanstack/react-router';
import {authClient} from '#/lib/auth-client';

const TopPage = () => {
  const {data: session, isPending} = authClient.useSession();

  if (isPending) {
    return (
      <Box sx={{display: 'flex', justifyContent: 'center', py: 10}}>
        <CircularProgress size={24}/>
      </Box>
    );
  }

  if (!session?.user) {
    return <Navigate to="/login" search={{redirect: '/'}} replace/>;
  }

  return (
    <Box sx={{display: 'flex', justifyContent: 'center', px: 2, py: 10}}>
      <Card sx={{width: '100%', maxWidth: 420}}>
        <CardContent>
          <Stack spacing={3}>
            <Box>
              <Typography variant="h6" fontWeight={600}>
                Welcome back
              </Typography>
              <Typography variant="body2" color="text.secondary">
                You're signed in as {session.user.email}
              </Typography>
            </Box>

            <Stack direction="row" spacing={1.5} alignItems="center">
              {session.user.image ? (
                <Avatar
                  src={session.user.image}
                  alt={session.user.name || 'User'}
                  sx={{width: 40, height: 40}}
                />
              ) : (
                <Avatar sx={{width: 40, height: 40}}>
                  {(session.user.name?.charAt(0) || 'U').toUpperCase()}
                </Avatar>
              )}
              <Box sx={{minWidth: 0}}>
                <Typography variant="body2" fontWeight={600} noWrap>
                  {session.user.name}
                </Typography>
                <Typography variant="caption" color="text.secondary" noWrap>
                  {session.user.email}
                </Typography>
              </Box>
            </Stack>

            <Button
              variant="outlined"
              onClick={() => {
                void authClient.signOut();
              }}
            >
              Sign out
            </Button>
          </Stack>
        </CardContent>
      </Card>
    </Box>
  );
};

export const Route = createFileRoute('/')({component: TopPage});
